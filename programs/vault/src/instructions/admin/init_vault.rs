use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::constants::{
    MAX_COOLDOWN_DURATION, MIN_COOLDOWN_DURATION, VAULT_CONFIG_SEED,
    VAULT_SILO_USDU_TOKEN_ACCOUNT_SEED, VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED, VAULT_STATE_SEED,
    VAULT_SUSDU_TOKEN_ACCOUNT_SEED, VAULT_USDU_TOKEN_ACCOUNT_SEED,
};
use crate::error::VaultError;
use crate::events::{VaultConfigInitialized, VaultStateInitialized, VaultTokenAccountInitialized};
use crate::state::{VaultConfig, VaultState};
use guardian::constants::ACCESS_REGISTRY_SEED;
use guardian::state::AccessRegistry;

#[derive(Accounts)]
pub struct InitVaultConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    pub susdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [VAULT_CONFIG_SEED],
        bump,
        space = VaultConfig::SIZE,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitVaultState<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = VaultState::SIZE,
        seeds = [VAULT_STATE_SEED],
        bump,
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitVaultStateUsduTokenAccount<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
        constraint = vault_state.admin == admin.key() @ VaultError::VaultStateAdminMismatch,
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [VAULT_USDU_TOKEN_ACCOUNT_SEED],
        bump,
        token::mint = usdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitVaultStateSusduTokenAccount<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
        constraint = vault_state.admin == admin.key() @ VaultError::VaultStateAdminMismatch,
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [VAULT_SUSDU_TOKEN_ACCOUNT_SEED],
        bump,
        token::mint = susdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_susdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub susdu_token: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitVaultStateStakePoolUsduTokenAccount<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
        constraint = vault_state.admin == admin.key() @ VaultError::VaultStateAdminMismatch,
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED],
        bump,
        token::mint = usdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_stake_pool_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitVaultStateSlioUsduTokenAccount<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
        constraint = vault_state.admin == admin.key() @ VaultError::VaultStateAdminMismatch,
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [VAULT_SILO_USDU_TOKEN_ACCOUNT_SEED],
        bump,
        token::mint = usdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_silo_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_init_vault_config(
    ctx: Context<InitVaultConfig>,
    cooldown_duration: u64,
) -> Result<()> {
    let vault_config = &mut ctx.accounts.vault_config;
    require!(
        vault_config.is_initialized == false,
        VaultError::ConfigAlreadyInitialized
    );

    require!(
        cooldown_duration >= MIN_COOLDOWN_DURATION,
        VaultError::CooldownDurationTooShort
    );
    require!(
        cooldown_duration <= MAX_COOLDOWN_DURATION,
        VaultError::CooldownDurationTooLong
    );

    vault_config.admin = ctx.accounts.admin.key();
    vault_config.pending_admin = Pubkey::default();
    vault_config.cooldown_duration = cooldown_duration;
    vault_config.bump = ctx.bumps.vault_config;
    vault_config.is_initialized = true;
    vault_config.usdu = ctx.accounts.usdu_token.key();
    vault_config.susdu = ctx.accounts.susdu_token.key();
    vault_config.access_registry = ctx.accounts.access_registry.key();
    vault_config.vesting_amount = 0;
    vault_config.last_distribution_timestamp = 0;
    vault_config.total_staked_usdu_supply = 0;
    vault_config.total_cooldown_usdu_amount = 0;

    // Emit event
    emit!(VaultConfigInitialized {
        vault_config: vault_config.key(),
        admin: vault_config.admin,
        usdu_token: vault_config.usdu,
        susdu_token: vault_config.susdu,
        access_registry: vault_config.access_registry,
        cooldown_duration: vault_config.cooldown_duration,
    });

    Ok(())
}

pub(crate) fn process_init_vault_state(ctx: Context<InitVaultState>) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    require!(
        vault_state.is_initialized == false,
        VaultError::StateAlreadyInitialized
    );
    vault_state.is_initialized = true;
    vault_state.bump = ctx.bumps.vault_state;
    vault_state.admin = ctx.accounts.admin.key();

    // Emit event
    emit!(VaultStateInitialized {
        vault_state: vault_state.key(),
        admin: vault_state.admin,
    });

    Ok(())
}

pub fn process_init_vault_state_usdu_token_account(
    ctx: Context<InitVaultStateUsduTokenAccount>,
) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    require!(
        vault_state.vault_usdu_token_account == Pubkey::default(),
        VaultError::VaultUsduTokenAccountAlreadyInitialized
    );
    vault_state.vault_usdu_token_account = ctx.accounts.vault_usdu_token_account.key();
    vault_state.vault_usdu_token_account_bump = ctx.bumps.vault_usdu_token_account;

    // Emit event
    emit!(VaultTokenAccountInitialized {
        vault_state: vault_state.key(),
        token_account: vault_state.vault_usdu_token_account,
    });

    Ok(())
}

pub fn process_init_vault_state_susdu_token_account(
    ctx: Context<InitVaultStateSusduTokenAccount>,
) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    require!(
        vault_state.vault_susdu_token_account == Pubkey::default(),
        VaultError::VaultSusduTokenAccountAlreadyInitialized
    );
    vault_state.vault_susdu_token_account = ctx.accounts.vault_susdu_token_account.key();
    vault_state.vault_susdu_token_account_bump = ctx.bumps.vault_susdu_token_account;

    // Emit event
    emit!(VaultTokenAccountInitialized {
        vault_state: vault_state.key(),
        token_account: vault_state.vault_susdu_token_account,
    });

    Ok(())
}

pub fn process_init_vault_state_stake_pool_usdu_token_account(
    ctx: Context<InitVaultStateStakePoolUsduTokenAccount>,
) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    require!(
        vault_state.vault_stake_pool_usdu_token_account == Pubkey::default(),
        VaultError::VaultStakePoolUsduTokenAccountAlreadyInitialized
    );
    vault_state.vault_stake_pool_usdu_token_account =
        ctx.accounts.vault_stake_pool_usdu_token_account.key();
    vault_state.vault_stake_pool_usdu_token_account_bump =
        ctx.bumps.vault_stake_pool_usdu_token_account;

    // Emit event
    emit!(VaultTokenAccountInitialized {
        vault_state: vault_state.key(),
        token_account: vault_state.vault_stake_pool_usdu_token_account,
    });

    Ok(())
}

pub fn process_init_vault_state_slio_usdu_token_account(
    ctx: Context<InitVaultStateSlioUsduTokenAccount>,
) -> Result<()> {
    let vault_state = &mut ctx.accounts.vault_state;
    require!(
        vault_state.vault_silo_usdu_token_account == Pubkey::default(),
        VaultError::VaultSiloUsduTokenAccountAlreadyInitialized
    );
    vault_state.vault_silo_usdu_token_account = ctx.accounts.vault_silo_usdu_token_account.key();
    vault_state.vault_silo_usdu_token_account_bump = ctx.bumps.vault_silo_usdu_token_account;

    // Emit event
    emit!(VaultTokenAccountInitialized {
        vault_state: vault_state.key(),
        token_account: vault_state.vault_silo_usdu_token_account,
    });

    Ok(())
}
