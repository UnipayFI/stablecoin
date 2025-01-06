use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, TokenAccount, TransferChecked, Mint, transfer_checked};
use anchor_spl::associated_token::AssociatedToken;

use crate::state::{VaultConfig};
use crate::constants::{VAULT_CONFIG_SEED};
use crate::error::VaultError;
use crate::events::DepositCollateralMintUsduEvent;

use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};

use usdu::cpi::{mint_usdu, accounts::MintUsdu};
use usdu::program::Usdu;
use usdu::state::UsduConfig;
use usdu::constants::USDU_CONFIG_SEED;

#[derive(Accounts)]
pub struct DepositCollateralMintUsdu<'info> {
    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        mut,
        seeds = [USDU_CONFIG_SEED],
        bump = usdu_config.bump,
        seeds::program = usdu::id(),
    )]
    pub usdu_config: Box<Account<'info, UsduConfig>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), vault_config.key().as_ref(), Role::UsduMinter.to_seed().as_slice()],
        bump = usdu_minter.bump,
        seeds::program = guardian::id(),
    )]
    pub usdu_minter: Box<Account<'info, AccessRole>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), authority.key().as_ref(), Role::VaultUsduMinter.to_seed().as_slice()],
        bump = vault_usdu_minter.bump,
        seeds::program = guardian::id(),
    )]
    pub vault_usdu_minter: Box<Account<'info, AccessRole>>,

    /// CHECK: no need to checked
    pub benefactor: UncheckedAccount<'info>,
    /// CHECK: no need to checked
    pub beneficiary: UncheckedAccount<'info>,
    /// CHECK: no need to checked
    pub fund: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = collateral_token,
        associated_token::authority = benefactor,
        associated_token::token_program = token_program,
    )]
    pub benefactor_collateral_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = collateral_token,
        associated_token::authority = fund,
        associated_token::token_program = token_program,
    )]
    pub fund_collateral_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = collateral_token,
        associated_token::authority = vault_config,
        associated_token::token_program = token_program,
    )]
    pub vault_collateral_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = usdu_token,
        associated_token::authority = beneficiary,
        associated_token::token_program = token_program,
    )]
    pub beneficiary_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub collateral_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    
    pub usdu_program: Program<'info, Usdu>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_deposit_collateral_mint_usdu(
    ctx: Context<DepositCollateralMintUsdu>,
    collateral_amount: u64,
    usdu_amount: u64,
) -> Result<()> {
    let vault_config = &ctx.accounts.vault_config;
    require!(vault_config.is_initialized, VaultError::ConfigNotInitialized);

    // check vault_usdu_minter role
    require!(
        ctx.accounts.vault_usdu_minter.is_initialized,
        VaultError::AccessRoleNotInitialized
    );
    require!(
        ctx.accounts.vault_usdu_minter.access_registry.eq(&ctx.accounts.access_registry.key()),
        VaultError::AccessRegistryMismatch
    );

    // delegate amount checked
    // beneficiary should approve enough amount to the vault
    let delegate_amount = ctx.accounts.benefactor_collateral_token_account.delegated_amount;
    require!(delegate_amount >= collateral_amount, VaultError::InsufficientCollateral);
    require!(ctx.accounts.benefactor_collateral_token_account.delegate.is_some(), VaultError::NoDelegate);
    require!(
        ctx.accounts.benefactor_collateral_token_account.delegate.unwrap().eq(&ctx.accounts.vault_config.key()), 
        VaultError::DelegateAccountMismatch
    );

    // 1. transfer collateral from benefactor to fund
    let config_bump = &[vault_config.bump];
    let config_seeds = &[
        &[
            VAULT_CONFIG_SEED,
            config_bump,
        ][..],
    ];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.benefactor_collateral_token_account.to_account_info().clone(),
                to: ctx.accounts.fund_collateral_token_account.to_account_info().clone(),
                authority: ctx.accounts.vault_config.to_account_info().clone(),
                mint: ctx.accounts.collateral_token.to_account_info().clone(),
            },
            config_seeds,
        ),
        collateral_amount,
        ctx.accounts.collateral_token.decimals,
    )?;

    // 2. mint usdu to beneficiary
    mint_usdu(
        CpiContext::new_with_signer(
            ctx.accounts.usdu_program.to_account_info(),
            MintUsdu {
                authority: ctx.accounts.vault_config.to_account_info(),
                access_registry: ctx.accounts.access_registry.to_account_info(),
                access_role: ctx.accounts.usdu_minter.to_account_info(),
                usdu_config: ctx.accounts.usdu_config.to_account_info(),
                usdu_token: ctx.accounts.usdu_token.to_account_info(),
                beneficiary: ctx.accounts.beneficiary.to_account_info(),
                beneficiary_token_account: ctx.accounts.beneficiary_usdu_token_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            },
            config_seeds,
        ),
        usdu_amount,
    )?;

    emit!(DepositCollateralMintUsduEvent {
        benefactor: ctx.accounts.benefactor.key(),
        beneficiary: ctx.accounts.beneficiary.key(),
        fund: ctx.accounts.fund.key(),
        collateral_amount,
        usdu_amount,
    });

    Ok(())
}