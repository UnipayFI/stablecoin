use anchor_lang::prelude::*;

use crate::constants::SUSDU_CONFIG_SEED;
use crate::state::SusduConfig;
use crate::events::SusduConfigInitialized;
use crate::error::SusduError;

use guardian::state::AccessRegistry;
use guardian::constants::ACCESS_REGISTRY_SEED;

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
        space = SusduConfig::SIZE,
        seeds = [SUSDU_CONFIG_SEED],
        bump
    )]
    pub susdu_config: Account<'info, SusduConfig>,
    pub system_program: Program<'info, System>,
}

pub fn process_init_config(ctx: Context<InitConfig>) -> Result<()> {
    require!(!ctx.accounts.susdu_config.is_initialized, SusduError::ConfigAlreadyInitialized);

    ctx.accounts.susdu_config.admin = ctx.accounts.admin.key();
    ctx.accounts.susdu_config.access_registry = ctx.accounts.access_registry.key();
    ctx.accounts.susdu_config.is_initialized = true;
    ctx.accounts.susdu_config.bump = ctx.bumps.susdu_config;
    emit!(SusduConfigInitialized {
        susdu_config: ctx.accounts.susdu_config.key(),
        admin: ctx.accounts.admin.key(),
        access_registry: ctx.accounts.access_registry.key(),
    });
    Ok(())
}
