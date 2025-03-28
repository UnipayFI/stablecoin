use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, Token2022, TokenAccount, TransferChecked,
};

use crate::constants::{VAULT_CONFIG_SEED, VAULT_STATE_SEED, VAULT_USDU_TOKEN_ACCOUNT_SEED};
use crate::error::VaultError;
use crate::events::RedeemUsduWithdrawCollateralEvent;
use crate::state::{VaultConfig, VaultState};
use crate::utils::{get_transfer_inverse_fee, is_supported_mint};

use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::utils::has_role;

use usdu::constants::USDU_CONFIG_SEED;
use usdu::cpi::{accounts::RedeemUsdu, redeem_usdu};
use usdu::program::Usdu;
use usdu::state::UsduConfig;

#[derive(Accounts)]
pub struct RedeemUsduWithdrawCollateral<'info> {
    #[account(
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
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
    // config
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), vault_config.key().as_ref(), Role::UsduRedeemer.to_seed().as_slice()],
        bump = usdu_redeemer.bump,
        seeds::program = guardian::id(),
    )]
    pub usdu_redeemer: Box<Account<'info, AccessRole>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), authority.key().as_ref(), Role::CollateralWithdrawer.to_seed().as_slice()],
        bump = collateral_withdrawer.bump,
        seeds::program = guardian::id(),
    )]
    pub collateral_withdrawer: Box<Account<'info, AccessRole>>,

    /// CHECK: no need to checked
    pub benefactor: UncheckedAccount<'info>,
    /// CHECK: no need to checked
    pub beneficiary: UncheckedAccount<'info>,
    /// CHECK: no need to checked
    pub fund: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = usdu_token,
        associated_token::authority = beneficiary,
        associated_token::token_program = token_program,
    )]
    pub beneficiary_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// Collateral is withdrawn from fund_collateral_token_account
    /// This is part of the business requirement, where the fund account is controlled by a trusted entity
    #[account(
        mut,
        associated_token::mint = collateral_token,
        associated_token::authority = fund,
        associated_token::token_program = token_program,
    )]
    pub fund_collateral_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [VAULT_USDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_usdu_token_account_bump,
        token::mint = usdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = collateral_token,
        associated_token::authority = benefactor,
        associated_token::token_program = token_program,
    )]
    pub benefactor_collateral_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub collateral_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,

    pub usdu_program: Program<'info, Usdu>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_redeem_usdu_withdraw_collateral(
    ctx: Context<RedeemUsduWithdrawCollateral>,
    collateral_amount: u64,
    usdu_amount: u64,
) -> Result<()> {
    require!(
        collateral_amount > 0,
        VaultError::AmountMustBeGreaterThanZero
    );
    require!(usdu_amount > 0, VaultError::AmountMustBeGreaterThanZero);

    require!(
        is_supported_mint(&ctx.accounts.collateral_token)?,
        VaultError::CollateralMismatch
    );
    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.collateral_withdrawer.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            Role::CollateralWithdrawer,
        )?,
        VaultError::UnauthorizedRole
    );
    require!(
        ctx.accounts.vault_state.vault_usdu_token_account.key()
            == ctx.accounts.vault_usdu_token_account.key(),
        VaultError::InvalidVaultUsduTokenAccount
    );
    let vault_config = &ctx.accounts.vault_config;
    require!(
        vault_config.is_initialized,
        VaultError::ConfigNotInitialized
    );
    let vault_state = &ctx.accounts.vault_state;
    require!(vault_state.is_initialized, VaultError::StateNotInitialized);

    // can have TransferFeeConfig, but the fee must be 0
    let collateral_amount_with_fee = {
        let transfer_fee = get_transfer_inverse_fee(
            &ctx.accounts.collateral_token.to_account_info(),
            collateral_amount,
        )?;
        collateral_amount.checked_add(transfer_fee).unwrap()
    };
    require!(
        collateral_amount_with_fee == collateral_amount,
        VaultError::InvalidCollateralToken
    );

    // delegate amount checked
    // fund should approve enough collateral amount to the vault
    let delegate_collateral_amount = ctx.accounts.fund_collateral_token_account.delegated_amount;
    require!(
        delegate_collateral_amount >= collateral_amount,
        VaultError::InsufficientCollateral
    );
    require!(
        ctx.accounts
            .fund_collateral_token_account
            .delegate
            .is_some(),
        VaultError::NoDelegate
    );
    require!(
        ctx.accounts
            .fund_collateral_token_account
            .delegate
            .unwrap()
            .eq(&ctx.accounts.vault_config.key()),
        VaultError::DelegateAccountMismatch
    );

    // beneficiary should approve enough usdu amount to the vault
    let delegate_usdu_amount = ctx.accounts.beneficiary_usdu_token_account.delegated_amount;
    require!(
        delegate_usdu_amount >= usdu_amount,
        VaultError::InsufficientUsdu
    );
    require!(
        ctx.accounts
            .beneficiary_usdu_token_account
            .delegate
            .is_some(),
        VaultError::NoDelegate
    );
    require!(
        ctx.accounts
            .beneficiary_usdu_token_account
            .delegate
            .unwrap()
            .eq(&ctx.accounts.vault_config.key()),
        VaultError::DelegateAccountMismatch
    );

    // 1. transfer collateral from fund to benefactor
    let config_bump = &[vault_config.bump];
    let signer_seeds = &[&[VAULT_CONFIG_SEED, config_bump][..]];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx
                    .accounts
                    .fund_collateral_token_account
                    .to_account_info()
                    .clone(),
                to: ctx
                    .accounts
                    .benefactor_collateral_token_account
                    .to_account_info()
                    .clone(),
                authority: ctx.accounts.vault_config.to_account_info().clone(),
                mint: ctx.accounts.collateral_token.to_account_info().clone(),
            },
            signer_seeds,
        ),
        collateral_amount,
        ctx.accounts.collateral_token.decimals,
    )?;

    // 2. transfer usdu from beneficiary to vault
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx
                    .accounts
                    .beneficiary_usdu_token_account
                    .to_account_info()
                    .clone(),
                to: ctx
                    .accounts
                    .vault_usdu_token_account
                    .to_account_info()
                    .clone(),
                authority: ctx.accounts.vault_config.to_account_info().clone(),
                mint: ctx.accounts.usdu_token.to_account_info().clone(),
            },
            signer_seeds,
        ),
        usdu_amount,
        ctx.accounts.usdu_token.decimals,
    )?;

    // 3. redeem usdu
    redeem_usdu(
        CpiContext::new_with_signer(
            ctx.accounts.usdu_program.to_account_info(),
            RedeemUsdu {
                caller: ctx.accounts.vault_config.to_account_info(),
                access_registry: ctx.accounts.access_registry.to_account_info(),
                access_role: ctx.accounts.usdu_redeemer.to_account_info(),
                usdu_config: ctx.accounts.usdu_config.to_account_info(),
                usdu_token: ctx.accounts.usdu_token.to_account_info(),
                caller_token_account: ctx.accounts.vault_usdu_token_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            signer_seeds,
        ),
        usdu_amount,
    )?;

    emit!(RedeemUsduWithdrawCollateralEvent {
        benefactor: ctx.accounts.benefactor.key(),
        beneficiary: ctx.accounts.beneficiary.key(),
        fund: ctx.accounts.fund.key(),
        collateral_amount,
        usdu_amount,
    });

    Ok(())
}
