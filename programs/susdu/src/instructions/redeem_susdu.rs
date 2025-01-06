use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, Mint, TokenAccount};
use anchor_spl::token_2022::{burn, Burn};
use crate::constants::{SUSDU_CONFIG_SEED};
use crate::error::SusduError;
use crate::state::SusduConfig;
use crate::events::SusduTokenRedeemed;

use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};

#[derive(Accounts)]
pub struct RedeemSusdu<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

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
            authority.key().as_ref(),
            Role::SusduRedeemer.to_seed().as_slice(),
        ],
        bump = access_role.bump,
        seeds::program = guardian::id(),
    )]
    pub access_role: Box<Account<'info, AccessRole>>,
    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,
    #[account(mut)]
    pub susdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub authority_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_redeem_susdu(ctx: Context<RedeemSusdu>, susdu_amount: u64) -> Result<()> {
    require!(ctx.accounts.susdu_config.is_susdu_token_initialized, SusduError::ConfigNotSetupSusdu);
    require!(ctx.accounts.susdu_config.total_supply >= susdu_amount, SusduError::InsufficientSusdu);

    // check access role
    require!(ctx.accounts.access_role.is_initialized, SusduError::AccessRoleNotInitialized);
    require!(ctx.accounts.access_role.access_registry.eq(&ctx.accounts.access_registry.key()), SusduError::AccessRegistryMismatch);

    let susdu_config = &mut ctx.accounts.susdu_config;
    susdu_config.total_supply -= susdu_amount;
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.susdu_token.to_account_info(),
                from: ctx.accounts.authority_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        susdu_amount,
    )?;
    emit!(SusduTokenRedeemed {
        susdu_token: ctx.accounts.susdu_token.key(),
        amount: susdu_amount,
        authority: ctx.accounts.authority.key(),
        authority_token_account: ctx.accounts.authority_token_account.key(),
    });
    Ok(())
}
