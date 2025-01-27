use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, Mint, TokenAccount};
use anchor_spl::token_2022::{mint_to, MintTo};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::{USDU_CONFIG_SEED, USDU_SEED};
use crate::error::UsduError;
use crate::state::UsduConfig;
use crate::events::UsduTokenMinted;

use guardian::utils::has_role;
use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};

#[derive(Accounts)]
pub struct MintUsdu<'info> {
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
            Role::UsduMinter.to_seed().as_slice(),
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
    /// CHECK: no need to be checked
    pub receiver: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = usdu_token,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_mint_usdu(ctx: Context<MintUsdu>, usdu_amount: u64) -> Result<()> {
    require!(ctx.accounts.usdu_config.is_usdu_token_initialized, UsduError::ConfigNotSetupUSDU);
    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.access_role.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            Role::UsduMinter,
        )?,
        UsduError::UnauthorizedRole
    );
    let usdu_config = &mut ctx.accounts.usdu_config;
    usdu_config.total_supply += usdu_amount;
    let signer_seeds: &[&[&[u8]]] = &[&[USDU_SEED, &[ctx.accounts.usdu_config.usdu_token_bump]]];
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.usdu_token.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.usdu_token.to_account_info(),
            },
            signer_seeds,
        ),
        usdu_amount,
    )?;
    emit!(UsduTokenMinted {
        usdu_token: ctx.accounts.usdu_token.key(),
        amount: usdu_amount,
        receiver: ctx.accounts.receiver.key(),
        receiver_token_account: ctx.accounts.receiver_token_account.key(),
    });
    Ok(())
}
