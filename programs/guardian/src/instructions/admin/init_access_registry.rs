use anchor_lang::prelude::*;

use crate::state::AccessRegistry;
use crate::constants::ACCESS_REGISTRY_SEED;
use crate::error::GuardianError;
use crate::events::AccessRegistryInitialized;

#[derive(Accounts)]
pub struct InitAccessRegistry<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        space = AccessRegistry::SIZE,
        seeds = [ACCESS_REGISTRY_SEED],
        bump
    )]
    pub access_registry: Account<'info, AccessRegistry>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn process_init_access_registry(ctx: Context<InitAccessRegistry>) -> Result<()> {
    require!(
        !ctx.accounts.access_registry.is_initialized,
        GuardianError::AccessRegistryAlreadyInitialized
    );
    ctx.accounts.access_registry.admin = ctx.accounts.admin.key();
    ctx.accounts.access_registry.bump = ctx.bumps.access_registry;
    ctx.accounts.access_registry.is_initialized = true;
    emit!(AccessRegistryInitialized {
        access_registry: ctx.accounts.access_registry.key(),
    });
    Ok(())
}
