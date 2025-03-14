use anchor_lang::prelude::*;

use crate::constants::SUSDU_CONFIG_SEED;
use crate::error::SusduError;
use crate::events::{AdminTransferCompleted, AdminTransferProposed};
use crate::state::SusduConfig;

#[derive(Accounts)]
pub struct ProposeNewAdmin<'info> {
    #[account(mut)]
    pub current_admin: Signer<'info>,

    /// CHECK: This is the proposed new admin, no signature required
    pub proposed_admin: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
        constraint = susdu_config.admin == current_admin.key() @ SusduError::OnlyAdminCanProposeNewAdmin,
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptAdminTransfer<'info> {
    #[account(mut)]
    pub new_admin: Signer<'info>,

    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
        constraint = susdu_config.pending_admin == new_admin.key() @ SusduError::OnlyProposedAdminCanAccept,
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,

    pub system_program: Program<'info, System>,
}

pub fn process_propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
    let susdu_config = &mut ctx.accounts.susdu_config;

    require!(
        susdu_config.pending_admin != ctx.accounts.proposed_admin.key(),
        SusduError::ProposedAdminAlreadySet
    );

    require!(
        susdu_config.admin != ctx.accounts.proposed_admin.key(),
        SusduError::ProposedAdminIsCurrentAdmin
    );

    susdu_config.pending_admin = ctx.accounts.proposed_admin.key();

    emit!(AdminTransferProposed {
        susdu_config: susdu_config.key(),
        current_admin: ctx.accounts.current_admin.key(),
        proposed_admin: ctx.accounts.proposed_admin.key(),
    });

    Ok(())
}

pub fn process_accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
    let susdu_config = &mut ctx.accounts.susdu_config;

    require!(
        susdu_config.pending_admin != Pubkey::default(),
        SusduError::NoPendingAdminTransfer
    );

    let previous_admin = susdu_config.admin;

    susdu_config.admin = ctx.accounts.new_admin.key();
    susdu_config.pending_admin = Pubkey::default();

    emit!(AdminTransferCompleted {
        susdu_config: susdu_config.key(),
        previous_admin,
        new_admin: ctx.accounts.new_admin.key(),
    });

    Ok(())
}
