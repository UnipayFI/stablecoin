use anchor_lang::prelude::*;

use anchor_spl::token_interface::{transfer_hook_update, Mint, Token2022, TransferHookUpdate};

use crate::constants::{SUSDU_CONFIG_SEED, SUSDU_SEED};
use crate::error::SusduError;
use crate::events::TransferHookUpdated;
use crate::state::SusduConfig;

#[derive(Accounts)]
pub struct UpdateTransferHook<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
        constraint = susdu_config.admin == admin.key() @ SusduError::InvalidAdminAuthority,
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,
    #[account(mut)]
    pub susdu_token: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_update_transfer_hook(
    ctx: Context<UpdateTransferHook>,
    transfer_hook_program_id: Pubkey,
) -> Result<()> {
    let susdu_config = &mut ctx.accounts.susdu_config;
    let susdu_token = &ctx.accounts.susdu_token;

    let signed_seeds: &[&[&[u8]]] = &[&[SUSDU_SEED, &[susdu_config.susdu_token_bump]]];
    transfer_hook_update(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferHookUpdate {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: susdu_token.to_account_info(),
                authority: susdu_config.to_account_info(),
            },
            signed_seeds,
        ),
        Some(transfer_hook_program_id),
    )?;

    susdu_config.blacklist_hook_program_id = transfer_hook_program_id;

    emit!(TransferHookUpdated {
        susdu_config: susdu_config.key(),
        old_transfer_hook_program_id: susdu_config.blacklist_hook_program_id,
        new_transfer_hook_program_id: transfer_hook_program_id,
    });

    Ok(())
}
