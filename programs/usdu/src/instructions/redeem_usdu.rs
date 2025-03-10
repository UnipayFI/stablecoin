use anchor_lang::prelude::*;
use anchor_spl::token_2022::{burn, Burn};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::constants::USDU_CONFIG_SEED;
use crate::error::UsduError;
use crate::events::UsduTokenRedeemed;
use crate::state::UsduConfig;

use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::utils::has_role;

#[derive(Accounts)]
pub struct RedeemUsdu<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,

    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [
            ACCESS_ROLE_SEED,
            access_registry.key().as_ref(),
            caller.key().as_ref(),
            Role::UsduRedeemer.to_seed().as_slice(),
        ],
        bump = access_role.bump,
        seeds::program = guardian::id(),
    )]
    pub access_role: Box<Account<'info, AccessRole>>,
    #[account(
        mut,
        seeds = [USDU_CONFIG_SEED],
        bump = usdu_config.bump,
    )]
    pub usdu_config: Box<Account<'info, UsduConfig>>,
    #[account(mut)]
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub caller_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_redeem_usdu(ctx: Context<RedeemUsdu>, usdu_amount: u64) -> Result<()> {
    require!(
        ctx.accounts.usdu_config.is_usdu_token_initialized,
        UsduError::ConfigNotSetupUsdu
    );

    require!(
        ctx.accounts.usdu_token.key() == ctx.accounts.usdu_config.usdu_token,
        UsduError::InvalidUsduToken
    );

    require!(usdu_amount > 0, UsduError::AmountMustBeGreaterThanZero);
    require!(
        ctx.accounts.usdu_config.total_supply >= usdu_amount,
        UsduError::InsufficientUsdu
    );

    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.access_role.to_account_info(),
            &ctx.accounts.caller.to_account_info(),
            Role::UsduRedeemer,
        )?,
        UsduError::UnauthorizedRole
    );

    let usdu_config = &mut ctx.accounts.usdu_config;
    usdu_config.total_supply -= usdu_amount;
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.usdu_token.to_account_info(),
                from: ctx.accounts.caller_token_account.to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
            },
        ),
        usdu_amount,
    )?;
    emit!(UsduTokenRedeemed {
        usdu_token: ctx.accounts.usdu_token.key(),
        amount: usdu_amount,
        caller: ctx.accounts.caller.key(),
        caller_token_account: ctx.accounts.caller_token_account.key(),
    });
    Ok(())
}
