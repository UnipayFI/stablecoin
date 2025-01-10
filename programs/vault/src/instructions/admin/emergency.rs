use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    Mint,
    TokenAccount,
    Token2022,
};
use anchor_spl::token_2022::{
    TransferChecked,
    transfer_checked,
};
use anchor_spl::associated_token::AssociatedToken;

use crate::state::{VaultConfig, VaultState};
use crate::constants::{
    VAULT_CONFIG_SEED,
    VAULT_STATE_SEED,
    VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED,
    VAULT_SLIO_USDU_TOKEN_ACCOUNT_SEED,
    VAULT_SUSDU_TOKEN_ACCOUNT_SEED,
    VAULT_USDU_TOKEN_ACCOUNT_SEED,
};
use crate::error::VaultError;
use crate::utils::has_role_or_admin;

use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use guardian::state::{AccessRegistry, AccessRole, Role};

#[derive(Accounts)]
pub struct EmergencyWithdrawVaultStakePoolUsdu<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: no need to be checked
    pub receiver: UncheckedAccount<'info>,
    #[account(
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
        constraint = vault_config.admin == authority.key() @ VaultError::InvalidVaultAdminAuthority,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        mut,
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
        has_one = vault_stake_pool_usdu_token_account
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        seeds = [VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_stake_pool_usdu_token_account_bump,
    )]
    pub vault_stake_pool_usdu_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        associated_token::mint = usdu_token,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_usdu_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), authority.key().as_ref(), Role::VaultManager.to_seed().as_slice()],
        bump = vault_manager.bump,
        seeds::program = guardian::id(),
    )]
    pub vault_manager: Box<Account<'info, AccessRole>>,
    pub usdu_token: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EmergencyWithdrawVaultSlioUsdu<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: no need to be checked
    pub receiver: UncheckedAccount<'info>,
    #[account(
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
        constraint = vault_config.admin == authority.key() @ VaultError::InvalidVaultAdminAuthority,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        mut,
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
        has_one = vault_slio_usdu_token_account
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        seeds = [VAULT_SLIO_USDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_slio_usdu_token_account_bump,
    )]
    pub vault_slio_usdu_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        associated_token::mint = usdu_token,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_usdu_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), authority.key().as_ref(), Role::VaultManager.to_seed().as_slice()],
        bump = vault_manager.bump,
        seeds::program = guardian::id(),
    )]
    pub vault_manager: Box<Account<'info, AccessRole>>,
    pub usdu_token: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EmergencyWithdrawVaultUsdu<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: no need to be checked
    pub receiver: UncheckedAccount<'info>,
    #[account(
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
        constraint = vault_config.admin == authority.key() @ VaultError::InvalidVaultAdminAuthority,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        mut,
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
        has_one = vault_usdu_token_account
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        seeds = [VAULT_USDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_usdu_token_account_bump,
    )]
    pub vault_usdu_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        associated_token::mint = usdu_token,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_usdu_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), authority.key().as_ref(), Role::VaultManager.to_seed().as_slice()],
        bump = vault_manager.bump,
        seeds::program = guardian::id(),
    )]
    pub vault_manager: Box<Account<'info, AccessRole>>,
    pub usdu_token: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct EmergencyWithdrawVaultSusdu<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: no need to be checked
    pub receiver: UncheckedAccount<'info>,
    #[account(
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
        constraint = vault_config.admin == authority.key() @ VaultError::InvalidVaultAdminAuthority,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        mut,
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
        has_one = vault_susdu_token_account
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        seeds = [VAULT_SUSDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_susdu_token_account_bump,
    )]
    pub vault_susdu_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        associated_token::mint = susdu_token,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_susdu_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), authority.key().as_ref(), Role::VaultManager.to_seed().as_slice()],
        bump = vault_manager.bump,
        seeds::program = guardian::id(),
    )]
    pub vault_manager: Box<Account<'info, AccessRole>>,
    pub susdu_token: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

pub(crate) fn process_emergency_withdraw_vault_stake_pool_usdu(
    ctx: Context<EmergencyWithdrawVaultStakePoolUsdu>,
    amount: u64,
) -> Result<()> {
    require!(
        has_role_or_admin(
            &ctx.accounts.vault_config, 
            &ctx.accounts.access_registry, 
            &ctx.accounts.vault_manager, 
            &ctx.accounts.authority, 
            Role::VaultManager
        )?, 
        VaultError::UnauthorizedRole
    );
    require!(amount > 0, VaultError::AmountMustBeGreaterThanZero);
    require!(ctx.accounts.vault_stake_pool_usdu_token_account.amount >= amount, VaultError::InsufficientStakePoolUsdu);
    let vault_state = &ctx.accounts.vault_state;
    let vault_stake_pool_usdu_token_account_bump = &[vault_state.vault_stake_pool_usdu_token_account_bump];
    let vault_stake_pool_usdu_token_account_seed = &[
        &[
            VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED,
            vault_stake_pool_usdu_token_account_bump,
        ][..],
    ];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_stake_pool_usdu_token_account.to_account_info(),
                to: ctx.accounts.receiver_usdu_token_account.to_account_info(),
                authority: ctx.accounts.vault_stake_pool_usdu_token_account.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
            },
            vault_stake_pool_usdu_token_account_seed,
        ),
        amount,
        ctx.accounts.usdu_token.decimals,
    )?;
    Ok(())
}

pub(crate) fn process_emergency_withdraw_vault_slio_usdu(
    ctx: Context<EmergencyWithdrawVaultSlioUsdu>,
    amount: u64,
) -> Result<()> {
    require!(
        has_role_or_admin(
            &ctx.accounts.vault_config, 
            &ctx.accounts.access_registry, 
            &ctx.accounts.vault_manager, 
            &ctx.accounts.authority, 
            Role::VaultManager
        )?, 
        VaultError::UnauthorizedRole
    );
    require!(amount > 0, VaultError::AmountMustBeGreaterThanZero);
    require!(ctx.accounts.vault_slio_usdu_token_account.amount >= amount, VaultError::InsufficientSlioUsdu);
    let vault_state = &ctx.accounts.vault_state;
    let vault_slio_usdu_token_account_bump = &[vault_state.vault_slio_usdu_token_account_bump];
    let vault_slio_usdu_token_account_seed = &[
        &[
            VAULT_SLIO_USDU_TOKEN_ACCOUNT_SEED,
            vault_slio_usdu_token_account_bump,
        ][..],
    ];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_slio_usdu_token_account.to_account_info(),
                to: ctx.accounts.receiver_usdu_token_account.to_account_info(),
                authority: ctx.accounts.vault_slio_usdu_token_account.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
            },
            vault_slio_usdu_token_account_seed,
        ),
        amount,
        ctx.accounts.usdu_token.decimals,
    )?;
    Ok(())
}

pub(crate) fn process_emergency_withdraw_vault_usdu(
    ctx: Context<EmergencyWithdrawVaultUsdu>,
    amount: u64,
) -> Result<()> {
    require!(
        has_role_or_admin(
            &ctx.accounts.vault_config, 
            &ctx.accounts.access_registry, 
            &ctx.accounts.vault_manager, 
            &ctx.accounts.authority, 
            Role::VaultManager
        )?, 
        VaultError::UnauthorizedRole
    );
    require!(amount > 0, VaultError::AmountMustBeGreaterThanZero);
    require!(ctx.accounts.vault_usdu_token_account.amount >= amount, VaultError::InsufficientVaultUsdu);
    let vault_state = &ctx.accounts.vault_state;
    let vault_usdu_token_account_bump = &[vault_state.vault_usdu_token_account_bump];
    let vault_usdu_token_account_seed = &[
        &[
            VAULT_USDU_TOKEN_ACCOUNT_SEED,
            vault_usdu_token_account_bump,
        ][..],
    ];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_usdu_token_account.to_account_info(),
                to: ctx.accounts.receiver_usdu_token_account.to_account_info(),
                authority: ctx.accounts.vault_usdu_token_account.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
            },
            vault_usdu_token_account_seed,
        ),
        amount,
        ctx.accounts.usdu_token.decimals,
    )?;
    Ok(())
}   

pub(crate) fn process_emergency_withdraw_vault_susdu(
    ctx: Context<EmergencyWithdrawVaultSusdu>,
    amount: u64,
) -> Result<()> {
    require!(
        has_role_or_admin(
            &ctx.accounts.vault_config, 
            &ctx.accounts.access_registry, 
            &ctx.accounts.vault_manager, 
            &ctx.accounts.authority, 
            Role::VaultManager
        )?, 
        VaultError::UnauthorizedRole
    );
    require!(amount > 0, VaultError::AmountMustBeGreaterThanZero);
    require!(ctx.accounts.vault_susdu_token_account.amount >= amount, VaultError::InsufficientVaultSusdu);
    let vault_state = &ctx.accounts.vault_state;
    let vault_susdu_token_account_bump = &[vault_state.vault_susdu_token_account_bump];
    let vault_susdu_token_account_seed = &[
        &[
            VAULT_SUSDU_TOKEN_ACCOUNT_SEED,
            vault_susdu_token_account_bump,
        ][..],
    ];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_susdu_token_account.to_account_info(),
                to: ctx.accounts.receiver_susdu_token_account.to_account_info(),
                authority: ctx.accounts.vault_susdu_token_account.to_account_info(),
                mint: ctx.accounts.susdu_token.to_account_info(),
            },
            vault_susdu_token_account_seed,
        ),
        amount,
        ctx.accounts.susdu_token.decimals,
    )?;
    Ok(())
}   