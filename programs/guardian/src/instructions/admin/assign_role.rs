use anchor_lang::prelude::*;

use crate::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use crate::error::GuardianError;
use crate::events::AccessRoleAssigned;
use crate::state::{AccessRegistry, AccessRole, Role};
use crate::utils::has_role;
#[derive(Accounts)]
#[instruction(role: Role)]
pub struct AssignRole<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// CHECK: no need to be checked
    pub user: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [ACCESS_REGISTRY_SEED],
        bump = access_registry.bump,
        constraint = access_registry.is_initialized @ GuardianError::AccessRegistryNotInitialized,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        init_if_needed,
        payer = authority,
        space = AccessRole::SIZE,
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), user.key().as_ref(), role.to_seed().as_slice()],
        bump
    )]
    pub assign_role: Box<Account<'info, AccessRole>>,
    /// CHECK: will be checked in the instruction
    #[account(mut)]
    pub guardian_admin: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn process_assign_role(ctx: Context<AssignRole>, role: Role) -> Result<()> {
    require!(
        !ctx.accounts.assign_role.is_initialized,
        GuardianError::AccessRoleAlreadyInitialized
    );
    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.guardian_admin.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            Role::GuardianAdmin,
        )?,
        GuardianError::InvalidRightToAssignRole
    );
    ctx.accounts.assign_role.role = role;
    ctx.accounts.assign_role.is_initialized = true;
    ctx.accounts.assign_role.owner = ctx.accounts.user.key();
    ctx.accounts.assign_role.access_registry = ctx.accounts.access_registry.key();
    ctx.accounts.assign_role.bump = ctx.bumps.assign_role;
    emit!(AccessRoleAssigned {
        role,
        address: ctx.accounts.user.key(),
    });
    Ok(())
}
