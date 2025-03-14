use anchor_lang::prelude::*;

use crate::constants::{VAULT_CONFIG_SEED, VAULT_STATE_SEED};
use crate::error::VaultError;
use crate::events::{AdminTransferCompleted, AdminTransferProposed};
use crate::state::{VaultConfig, VaultState};

#[derive(Accounts)]
pub struct ProposeNewAdmin<'info> {
    #[account(mut)]
    pub current_admin: Signer<'info>,

    /// CHECK: This is the proposed new admin, no signature required
    pub proposed_admin: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
        constraint = vault_config.admin == current_admin.key() @ VaultError::OnlyAdminCanProposeNewAdmin,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptAdminTransfer<'info> {
    #[account(mut)]
    pub new_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
        constraint = vault_config.pending_admin == new_admin.key() @ VaultError::OnlyProposedAdminCanAccept,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,

    #[account(
        mut,
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
    )]
    pub vault_state: Box<Account<'info, VaultState>>,

    pub system_program: Program<'info, System>,
}

pub fn process_propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
    let vault_config = &mut ctx.accounts.vault_config;

    require!(
        vault_config.pending_admin != ctx.accounts.proposed_admin.key(),
        VaultError::ProposedAdminAlreadySet
    );

    require!(
        vault_config.admin != ctx.accounts.proposed_admin.key(),
        VaultError::ProposedAdminIsCurrentAdmin
    );

    vault_config.pending_admin = ctx.accounts.proposed_admin.key();

    emit!(AdminTransferProposed {
        vault_config: vault_config.key(),
        current_admin: ctx.accounts.current_admin.key(),
        proposed_admin: ctx.accounts.proposed_admin.key(),
    });

    Ok(())
}

pub fn process_accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
    let vault_config = &mut ctx.accounts.vault_config;
    let vault_state = &mut ctx.accounts.vault_state;

    require!(
        vault_config.pending_admin != Pubkey::default(),
        VaultError::NoPendingAdminTransfer
    );

    let previous_admin = vault_config.admin;

    vault_config.admin = ctx.accounts.new_admin.key();
    vault_config.pending_admin = Pubkey::default();

    vault_state.admin = ctx.accounts.new_admin.key();

    emit!(AdminTransferCompleted {
        vault_config: vault_config.key(),
        previous_admin,
        new_admin: ctx.accounts.new_admin.key(),
    });

    Ok(())
}
