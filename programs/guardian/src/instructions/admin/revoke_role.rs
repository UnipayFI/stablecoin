use anchor_lang::prelude::*;

use crate::state::{AccessRegistry, AccessRole, Role};
use crate::constants::{ACCESS_REGISTRY_SEED};
use crate::error::GuardianError;
use crate::events::AccessRoleRevoked;
use crate::utils::has_role;

#[derive(Accounts)]
pub struct RevokeRole<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [ACCESS_REGISTRY_SEED],
        bump = access_registry.bump,
        constraint = access_registry.is_initialized @ GuardianError::AccessRegistryNotInitialized,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        mut,
        close = authority,
        constraint = revoke_role.is_initialized @ GuardianError::AccessRoleNotInitialized,
        constraint = revoke_role.access_registry == access_registry.key() @ GuardianError::MustBeAccessRegistry
    )]
    pub revoke_role: Box<Account<'info, AccessRole>>,
    /// CHECK: will be checked in the instruction
    #[account(mut)]
    pub guardian_admin: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn process_revoke_role(
    ctx: Context<RevokeRole>,
) -> Result<()> {
    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.guardian_admin.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            Role::GuardianAdmin,
        )?,
        GuardianError::InvalidRightToRevokeRole
    );
    // close the access role account
    ctx.accounts.revoke_role.is_initialized = false;
    emit!(AccessRoleRevoked {
        role: ctx.accounts.revoke_role.role,
        address: ctx.accounts.revoke_role.owner,
    });
    Ok(())
}
