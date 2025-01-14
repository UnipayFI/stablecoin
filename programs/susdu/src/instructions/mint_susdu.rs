use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, Mint, TokenAccount};
use anchor_spl::token_2022::{mint_to, MintTo};
use anchor_spl::associated_token::AssociatedToken;

use crate::constants::{SUSDU_CONFIG_SEED, SUSDU_SEED};
use crate::error::SusduError;
use crate::state::SusduConfig;
use crate::events::SusduTokenMinted;

use guardian::utils::has_role;
use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};

#[derive(Accounts)]
pub struct MintSusdu<'info> {
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
            Role::SusduMinter.to_seed().as_slice(),
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
    /// CHECK: no need to be checked
    pub receiver: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = susdu_token,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_mint_susdu(ctx: Context<MintSusdu>, susdu_amount: u64) -> Result<()> {
    require!(ctx.accounts.susdu_config.is_susdu_token_initialized, SusduError::ConfigNotSetupSusdu);

    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.access_role,
            &ctx.accounts.authority,
            Role::SusduMinter,
        )?,
        SusduError::UnauthorizedRole
    );

    let signer_seeds: &[&[&[u8]]] = &[&[SUSDU_SEED, &[ctx.accounts.susdu_config.susdu_token_bump]]];
    let susdu_config = &mut ctx.accounts.susdu_config;
    susdu_config.total_supply += susdu_amount;
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.susdu_token.to_account_info(),
                to: ctx.accounts.receiver_token_account.to_account_info(),
                authority: ctx.accounts.susdu_token.to_account_info(),
            },
            signer_seeds,
        ),
        susdu_amount,
    )?;
    emit!(SusduTokenMinted {
        susdu_token: ctx.accounts.susdu_token.key(),
        amount: susdu_amount,
        receiver: ctx.accounts.receiver.key(),
        receiver_token_account: ctx.accounts.receiver_token_account.key(),
    });
    Ok(())
}
