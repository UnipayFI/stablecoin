use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};

use crate::constants::{USDU_CONFIG_SEED, USDU_SEED};
use crate::state::UsduConfig;
use crate::error::UsduError;
use crate::events::UsduTokenCreated;

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateUsdu<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [USDU_CONFIG_SEED],
        bump = usdu_config.bump,
        has_one = admin,
    )]
    pub usdu_config: Box<Account<'info, UsduConfig>>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [USDU_SEED],
        bump,
        mint::freeze_authority = usdu_token,
        mint::authority = usdu_token,
        mint::decimals = decimals,
        mint::token_program = token_program,
    )]
    pub usdu_token: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_create_usdu(ctx: Context<CreateUsdu>, decimals: u8) -> Result<()> {
    require!(!ctx.accounts.usdu_config.is_usdu_token_initialized, UsduError::ConfigAlreadySetupUSDU);

    ctx.accounts.usdu_config.usdu_token = ctx.accounts.usdu_token.key();
    ctx.accounts.usdu_config.usdu_token_bump = ctx.bumps.usdu_token;
    ctx.accounts.usdu_config.is_usdu_token_initialized = true;
    ctx.accounts.usdu_config.total_supply = 0;
    emit!(UsduTokenCreated {
        usdu_token: ctx.accounts.usdu_token.key(),
        decimals,
    });
    Ok(())
}
