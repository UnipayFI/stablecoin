use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, TokenAccount, Mint};
use anchor_spl::token_2022::{
    TransferChecked,
    transfer_checked,
};
use anchor_spl::associated_token::AssociatedToken;

use crate::utils::is_blacklisted;
use crate::state::{VaultConfig, Cooldown, VaultState};
use crate::error::VaultError;
use crate::constants::{
    VAULT_STATE_SEED,
    VAULT_CONFIG_SEED,
    VAULT_COOLDOWN_SEED,
    VAULT_SLIO_USDU_TOKEN_ACCOUNT_SEED,
};

#[derive(Accounts)]
pub struct WithdrawUsdu<'info> {
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
     #[account(
         mut,
         associated_token::mint = usdu_token,
         associated_token::authority = receiver,
         associated_token::token_program = token_program,
     )]
     pub receiver_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [VAULT_SLIO_USDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_slio_usdu_token_account_bump,
        token::mint = usdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_slio_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [VAULT_COOLDOWN_SEED, usdu_token.key().as_ref(), receiver.key().as_ref()],
        bump = cooldown.bump,
    )]
    pub cooldown: Box<Account<'info, Cooldown>>,
    /// CHECK: will be checked in the process function
    pub blacklist_state: UncheckedAccount<'info>,
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_withdraw_usdu(ctx: Context<WithdrawUsdu>) -> Result<()> {  
    // 1. check blacklist
    require!(
        !is_blacklisted(&ctx.accounts.blacklist_state.to_account_info())?,
        VaultError::BlacklistAccount
    );
    // 2. check vault slio usdu token account
    require!(
        ctx.accounts.vault_state.vault_slio_usdu_token_account.key() == ctx.accounts.vault_slio_usdu_token_account.key(),
        VaultError::InvalidVaultSlioUsduTokenAccount
    );
    // 3. check cooldown
    require!(ctx.accounts.cooldown.is_initialized, VaultError::CooldownNotInitialized);
    require!(!ctx.accounts.cooldown.is_cooldown_active(), VaultError::CooldownActive);
    // 4. check receiver usdu token account
    require!(
        ctx.accounts.receiver_usdu_token_account.key() == ctx.accounts.cooldown.underlying_token_account,
        VaultError::InvalidReceiverUsduTokenAccount
    );
    let vault_config = &mut ctx.accounts.vault_config;
    let usdu_amount = ctx.accounts.cooldown.underlying_token_amount;
    let cooldown = &mut ctx.accounts.cooldown;
    cooldown.cooldown_end = 0;
    cooldown.underlying_token_amount = 0;
    // 5. check usdu amount
    require!(usdu_amount > 0, VaultError::InsufficientUsduCanNotBeZero);
    // 6. transfer usdu from vault_slio_usdu_token_account to caller_usdu_token_account
    let config_bump = &[vault_config.bump];
    let config_seeds = &[
        &[
            VAULT_CONFIG_SEED,
            config_bump,
        ][..],
    ];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_slio_usdu_token_account.to_account_info(),
                to: ctx.accounts.receiver_usdu_token_account.to_account_info(),
                authority: ctx.accounts.vault_config.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
            },
            config_seeds,
        ),
        usdu_amount,
        ctx.accounts.usdu_token.decimals,
    )?;
    Ok(())
}
