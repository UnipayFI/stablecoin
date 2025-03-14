use anchor_lang::prelude::*;

use crate::constants::ACCESS_REGISTRY_SEED;
use crate::error::GuardianError;
use crate::events::{AdminTransferCompleted, AdminTransferProposed};
use crate::state::AccessRegistry;

#[derive(Accounts)]
pub struct ProposeNewAdmin<'info> {
    #[account(mut)]
    pub current_admin: Signer<'info>,

    /// CHECK: This is the proposed new admin, no signature required
    pub proposed_admin: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [ACCESS_REGISTRY_SEED],
        bump = access_registry.bump,
        constraint = access_registry.admin == current_admin.key() @ GuardianError::OnlyAdminCanProposeNewAdmin,
    )]
    pub access_registry: Account<'info, AccessRegistry>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptAdminTransfer<'info> {
    #[account(mut)]
    pub new_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [ACCESS_REGISTRY_SEED],
        bump = access_registry.bump,
        constraint = access_registry.pending_admin == new_admin.key() @ GuardianError::OnlyProposedAdminCanAccept,
    )]
    pub access_registry: Account<'info, AccessRegistry>,

    pub system_program: Program<'info, System>,
}

pub fn process_propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
    let access_registry = &mut ctx.accounts.access_registry;

    require!(
        access_registry.pending_admin != ctx.accounts.proposed_admin.key(),
        GuardianError::ProposedAdminAlreadySet
    );

    require!(
        access_registry.admin != ctx.accounts.proposed_admin.key(),
        GuardianError::ProposedAdminIsCurrentAdmin
    );

    access_registry.pending_admin = ctx.accounts.proposed_admin.key();

    emit!(AdminTransferProposed {
        access_registry: access_registry.key(),
        current_admin: ctx.accounts.current_admin.key(),
        proposed_admin: ctx.accounts.proposed_admin.key(),
    });

    Ok(())
}

pub fn process_accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
    let access_registry = &mut ctx.accounts.access_registry;

    require!(
        access_registry.pending_admin != Pubkey::default(),
        GuardianError::NoPendingAdminTransfer
    );

    let previous_admin = access_registry.admin;

    access_registry.admin = ctx.accounts.new_admin.key();
    access_registry.pending_admin = Pubkey::default();

    emit!(AdminTransferCompleted {
        access_registry: access_registry.key(),
        previous_admin,
        new_admin: ctx.accounts.new_admin.key(),
    });

    Ok(())
}
