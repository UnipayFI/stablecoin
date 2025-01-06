use anchor_lang::prelude::*;

use crate::state::{AccessRegistry, AccessRole};
use crate::constants::{ACCESS_REGISTRY_SEED};
use crate::error::GuardianError;
use crate::events::AccessRoleRevoked;

#[derive(Accounts)]
pub struct RevokeRole<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        has_one = admin,
        seeds = [ACCESS_REGISTRY_SEED],
        bump = access_registry.bump,
        constraint = access_registry.is_initialized @ GuardianError::AccessRegistryNotInitialized,
        constraint = access_registry.admin == admin.key() @ GuardianError::MustBeAccessRegistryAdmin
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        mut,
        close = admin,
        constraint = access_role.is_initialized @ GuardianError::AccessRoleNotInitialized,
        constraint = access_role.access_registry == access_registry.key() @ GuardianError::MustBeAccessRegistry
    )]
    pub access_role: Box<Account<'info, AccessRole>>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn process_revoke_role(ctx: Context<RevokeRole>) -> Result<()> {
    // close the access role account
    ctx.accounts.access_role.is_initialized = false;
    emit!(AccessRoleRevoked {
        role: ctx.accounts.access_role.role,
        address: ctx.accounts.access_role.owner,
    });
    Ok(())
}
