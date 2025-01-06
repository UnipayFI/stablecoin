use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};

use crate::constants::{SUSDU_CONFIG_SEED, SUSDU_SEED};
use crate::state::SusduConfig;
use crate::error::SusduError;
use crate::events::SusduTokenCreated;

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateSusdu<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
        has_one = admin,
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [SUSDU_SEED],
        bump,
        mint::freeze_authority = susdu_token,
        mint::authority = susdu_token,
        mint::decimals = decimals,
        mint::token_program = token_program,
    )]
    pub susdu_token: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_create_susdu(ctx: Context<CreateSusdu>, decimals: u8) -> Result<()> {
    require!(!ctx.accounts.susdu_config.is_susdu_token_initialized, SusduError::ConfigAlreadySetupSusdu);

    ctx.accounts.susdu_config.susdu_token = ctx.accounts.susdu_token.key();
    ctx.accounts.susdu_config.susdu_token_bump = ctx.bumps.susdu_token;
    ctx.accounts.susdu_config.is_susdu_token_initialized = true;
    ctx.accounts.susdu_config.total_supply = 0;
    emit!(SusduTokenCreated {
        susdu_token: ctx.accounts.susdu_token.key(),
        decimals,
    });
    Ok(())
}
