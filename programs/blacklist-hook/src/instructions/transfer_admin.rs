use anchor_lang::prelude::*;

use crate::constants::BLACKLIST_HOOK_CONFIG;
use crate::error::BlacklistHookError;
use crate::events::{AdminTransferCompleted, AdminTransferProposed};
use crate::state::BlacklistHookConfig;

#[derive(Accounts)]
pub struct ProposeNewAdmin<'info> {
    #[account(mut)]
    pub current_admin: Signer<'info>,

    /// CHECK: This is the proposed new admin, no signature required
    pub proposed_admin: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [BLACKLIST_HOOK_CONFIG.as_bytes()],
        bump = blacklist_hook_config.bump,
        constraint = blacklist_hook_config.admin == current_admin.key() @ BlacklistHookError::OnlyAdminCanProposeNewAdmin,
    )]
    pub blacklist_hook_config: Box<Account<'info, BlacklistHookConfig>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptAdminTransfer<'info> {
    #[account(mut)]
    pub new_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [BLACKLIST_HOOK_CONFIG.as_bytes()],
        bump = blacklist_hook_config.bump,
        constraint = blacklist_hook_config.pending_admin == new_admin.key() @ BlacklistHookError::OnlyProposedAdminCanAccept,
    )]
    pub blacklist_hook_config: Box<Account<'info, BlacklistHookConfig>>,

    pub system_program: Program<'info, System>,
}

pub fn process_propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
    let blacklist_hook_config = &mut ctx.accounts.blacklist_hook_config;

    blacklist_hook_config.pending_admin = ctx.accounts.proposed_admin.key();

    emit!(AdminTransferProposed {
        blacklist_hook_config: blacklist_hook_config.key(),
        current_admin: ctx.accounts.current_admin.key(),
        proposed_admin: ctx.accounts.proposed_admin.key(),
    });

    Ok(())
}

pub fn process_accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
    let blacklist_hook_config = &mut ctx.accounts.blacklist_hook_config;

    require!(
        blacklist_hook_config.pending_admin != Pubkey::default(),
        BlacklistHookError::NoPendingAdminTransfer
    );

    let previous_admin = blacklist_hook_config.admin;

    blacklist_hook_config.admin = ctx.accounts.new_admin.key();
    blacklist_hook_config.pending_admin = Pubkey::default();

    emit!(AdminTransferCompleted {
        blacklist_hook_config: blacklist_hook_config.key(),
        previous_admin,
        new_admin: ctx.accounts.new_admin.key(),
    });

    Ok(())
}
