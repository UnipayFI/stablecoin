use anchor_lang::prelude::*;
use anchor_spl::token_interface::{burn, mint_to, Burn, Mint, MintTo, Token2022, TokenAccount};

use crate::constants::{SUSDU_CONFIG_SEED, SUSDU_SEED};
use crate::error::SusduError;
use crate::events::SusduTokenRedistributed;
use crate::state::SusduConfig;

use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::utils::has_role;

#[derive(Accounts)]
pub struct RedistributeSusdu<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,
    #[account(mut)]
    pub susdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub locked_susdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: will be checked in the process function
    #[account(mut)]
    pub receiver_susdu_token_account: UncheckedAccount<'info>,

    // Access Registry
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
            Role::SusduDistributor.to_seed().as_slice(),
        ],
        bump = access_role.bump,
        seeds::program = guardian::id(),
    )]
    pub access_role: Box<Account<'info, AccessRole>>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/// Redistributes or burns SUSDU tokens from a locked account.
///
/// This function serves two purposes:
/// 1. If the receiver_susdu_token_account is set to the default public key (Pubkey::default()),
///    it will burn the specified amount of SUSDU tokens from the locked account and reduce the total supply.
/// 2. Otherwise, it will transfer the specified amount of SUSDU tokens from the locked account to the
///    receiver's account.
///
/// This dual functionality is particularly useful for handling blacklisted or frozen accounts,
/// allowing administrators to either redistribute tokens to legitimate users or burn tokens
/// that should be removed from circulation.
pub fn process_redistribute_susdu(
    ctx: Context<RedistributeSusdu>,
    receiver: Pubkey,
    amount: u64,
) -> Result<()> {
    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.access_role.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            Role::SusduDistributor,
        )?,
        SusduError::UnauthorizedRole
    );
    require!(
        ctx.accounts.susdu_token.key() == ctx.accounts.susdu_config.susdu_token,
        SusduError::InvalidSusduToken
    );
    require!(amount > 0, SusduError::AmountMustBeGreaterThanZero);

    let signed_seeds: &[&[&[u8]]] = &[&[SUSDU_SEED, &[ctx.accounts.susdu_config.susdu_token_bump]]];
    require!(
        ctx.accounts.locked_susdu_token_account.amount >= amount,
        SusduError::InsufficientSusdu
    );

    // burn susdu token from locked susdu token account
    burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.susdu_token.to_account_info(),
                from: ctx.accounts.locked_susdu_token_account.to_account_info(),
                authority: ctx.accounts.susdu_token.to_account_info(),
            },
            signed_seeds,
        ),
        amount,
    )?;

    // mint susdu token to receiver_susdu_token_account
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.susdu_token.to_account_info(),
                to: ctx.accounts.receiver_susdu_token_account.to_account_info(),
                authority: ctx.accounts.susdu_token.to_account_info(),
            },
            signed_seeds,
        ),
        amount,
    )?;

    // Emit event
    emit!(SusduTokenRedistributed {
        susdu_token: ctx.accounts.susdu_token.key(),
        amount,
        from: ctx.accounts.locked_susdu_token_account.owner,
        to: receiver,
    });

    Ok(())
}
