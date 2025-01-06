use anchor_lang::prelude::*;

use crate::state::{AccessRegistry, AccessRole, Role};
use crate::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use crate::error::GuardianError;
use crate::events::AccessRoleAssigned;

#[derive(Accounts)]
#[instruction(role: Role)]
pub struct AssignRole<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: no need to be checked
    pub user: UncheckedAccount<'info>,

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
        init_if_needed,
        payer = admin,
        space = AccessRole::SIZE,
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), user.key().as_ref(), role.to_seed().as_slice()],
        bump
    )]
    pub access_role: Box<Account<'info, AccessRole>>,
    pub system_program: Program<'info, System>,

}

pub(crate) fn process_assign_role(ctx: Context<AssignRole>, role: Role) -> Result<()> {
    require!(!ctx.accounts.access_role.is_initialized, GuardianError::AccessRoleAlreadyInitialized);
    ctx.accounts.access_role.role = role;
    ctx.accounts.access_role.is_initialized = true;
    ctx.accounts.access_role.owner = ctx.accounts.user.key();
    ctx.accounts.access_role.access_registry = ctx.accounts.access_registry.key();
    ctx.accounts.access_role.bump = ctx.bumps.access_role;
    emit!(AccessRoleAssigned {
        role,
        address: ctx.accounts.user.key(),
    });
    Ok(())
}


