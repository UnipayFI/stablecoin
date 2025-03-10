use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::constants::USDU_CONFIG_SEED;
use crate::error::UsduError;
use crate::events::AdminTransferCompleted;
use crate::state::UsduConfig;

#[derive(Accounts)]
pub struct AdminTransfer<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: no need to be checked
    pub from: UncheckedAccount<'info>,
    /// CHECK: no need to be checked
    pub to: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [USDU_CONFIG_SEED],
        bump = usdu_config.bump,
        constraint = usdu_config.admin == admin.key() @ UsduError::InvalidAdminAuthority,
    )]
    pub usdu_config: Box<Account<'info, UsduConfig>>,
    #[account(
        mut,
        associated_token::mint = usdu_token,
        associated_token::authority = from,
        associated_token::token_program = token_program,
    )]
    pub from_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = usdu_token,
        associated_token::authority = to,
        associated_token::token_program = token_program,
    )]
    pub to_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,

    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_admin_transfer(
    ctx: Context<AdminTransfer>,
    amount: u64,
) -> Result<()> {
    // 1. Check amount
    require!(amount > 0, UsduError::AmountMustBeGreaterThanZero);
    
    // 2. Check if from account has enough tokens
    require!(
        ctx.accounts.from_usdu_token_account.amount >= amount,
        UsduError::InsufficientTokenBalance
    );

    // 3. Transfer tokens from from_usdu_token_account to to_usdu_token_account
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.from_usdu_token_account.to_account_info(),
                to: ctx.accounts.to_usdu_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.usdu_token.decimals,
    )?;

    // 4. Emit event
    emit!(AdminTransferCompleted {
        usdu_config: ctx.accounts.usdu_config.key(),
        from: ctx.accounts.from.key(),
        to: ctx.accounts.to.key(),
        amount,
    });

    Ok(())
} 