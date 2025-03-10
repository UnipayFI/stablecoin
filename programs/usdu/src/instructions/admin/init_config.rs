use anchor_lang::prelude::*;

use crate::constants::USDU_CONFIG_SEED;
use crate::error::UsduError;
use crate::events::UsduConfigInitialized;
use crate::state::UsduConfig;

use guardian::constants::ACCESS_REGISTRY_SEED;
use guardian::state::AccessRegistry;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        init_if_needed,
        payer = admin,
        space = UsduConfig::SIZE,
        seeds = [USDU_CONFIG_SEED],
        bump
    )]
    pub usdu_config: Account<'info, UsduConfig>,
    pub system_program: Program<'info, System>,
}

pub fn process_init_config(ctx: Context<InitConfig>) -> Result<()> {
    require!(
        !ctx.accounts.usdu_config.is_initialized,
        UsduError::ConfigAlreadyInitialized
    );

    ctx.accounts.usdu_config.admin = ctx.accounts.admin.key();
    ctx.accounts.usdu_config.pending_admin = Pubkey::default();
    ctx.accounts.usdu_config.access_registry = ctx.accounts.access_registry.key();
    ctx.accounts.usdu_config.is_initialized = true;
    ctx.accounts.usdu_config.bump = ctx.bumps.usdu_config;
    emit!(UsduConfigInitialized {
        usdu_config: ctx.accounts.usdu_config.key(),
        admin: ctx.accounts.admin.key(),
        access_registry: ctx.accounts.access_registry.key(),
    });
    Ok(())
}
