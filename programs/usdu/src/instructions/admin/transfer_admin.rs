use anchor_lang::prelude::*;

use crate::constants::USDU_CONFIG_SEED;
use crate::error::UsduError;
use crate::events::{AdminTransferCompleted, AdminTransferProposed};
use crate::state::UsduConfig;

#[derive(Accounts)]
pub struct ProposeNewAdmin<'info> {
    #[account(mut)]
    pub current_admin: Signer<'info>,

    /// CHECK: This is the proposed new admin, no signature required
    pub proposed_admin: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [USDU_CONFIG_SEED],
        bump = usdu_config.bump,
        constraint = usdu_config.admin == current_admin.key() @ UsduError::OnlyAdminCanProposeNewAdmin,
    )]
    pub usdu_config: Box<Account<'info, UsduConfig>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptAdminTransfer<'info> {
    #[account(mut)]
    pub new_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [USDU_CONFIG_SEED],
        bump = usdu_config.bump,
        constraint = usdu_config.pending_admin == new_admin.key() @ UsduError::OnlyProposedAdminCanAccept,
    )]
    pub usdu_config: Box<Account<'info, UsduConfig>>,

    pub system_program: Program<'info, System>,
}

pub fn process_propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
    let usdu_config = &mut ctx.accounts.usdu_config;

    require!(
        usdu_config.pending_admin != ctx.accounts.proposed_admin.key(),
        UsduError::ProposedAdminAlreadySet
    );

    require!(
        usdu_config.admin != ctx.accounts.proposed_admin.key(),
        UsduError::ProposedAdminIsCurrentAdmin
    );

    usdu_config.pending_admin = ctx.accounts.proposed_admin.key();

    emit!(AdminTransferProposed {
        usdu_config: usdu_config.key(),
        current_admin: ctx.accounts.current_admin.key(),
        proposed_admin: ctx.accounts.proposed_admin.key(),
    });

    Ok(())
}

pub fn process_accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
    let usdu_config = &mut ctx.accounts.usdu_config;

    require!(
        usdu_config.pending_admin != Pubkey::default(),
        UsduError::NoPendingAdminTransfer
    );

    let previous_admin = usdu_config.admin;

    usdu_config.admin = ctx.accounts.new_admin.key();
    usdu_config.pending_admin = Pubkey::default();

    emit!(AdminTransferCompleted {
        usdu_config: usdu_config.key(),
        previous_admin,
        new_admin: ctx.accounts.new_admin.key(),
    });

    Ok(())
}
