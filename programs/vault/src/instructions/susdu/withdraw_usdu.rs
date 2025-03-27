use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::constants::{
    VAULT_CONFIG_SEED, VAULT_COOLDOWN_SEED, VAULT_SILO_USDU_TOKEN_ACCOUNT_SEED, VAULT_STATE_SEED,
};
use crate::error::VaultError;
use crate::state::{Cooldown, VaultConfig, VaultState};

#[derive(Accounts)]
pub struct WithdrawUsdu<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
    /// CHECK: no need to checked
    pub receiver: UncheckedAccount<'info>,
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
        seeds = [VAULT_SILO_USDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_silo_usdu_token_account_bump,
        token::mint = usdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_silo_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [VAULT_COOLDOWN_SEED, usdu_token.key().as_ref(), receiver.key().as_ref(), caller.key().as_ref()],
        bump = cooldown.bump,
    )]
    pub cooldown: Box<Account<'info, Cooldown>>,

    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_withdraw_usdu(ctx: Context<WithdrawUsdu>) -> Result<()> {
    // 1. check vault slio usdu token account
    require!(
        ctx.accounts.vault_state.vault_silo_usdu_token_account.key()
            == ctx.accounts.vault_silo_usdu_token_account.key(),
        VaultError::InvalidVaultSiloUsduTokenAccount
    );
    // 2. check cooldown
    require!(
        ctx.accounts.cooldown.is_initialized,
        VaultError::CooldownNotInitialized
    );
    require!(
        !ctx.accounts.cooldown.is_cooldown_active(),
        VaultError::CooldownActive
    );
    require!(
        ctx.accounts.cooldown.owner == ctx.accounts.caller.key(),
        VaultError::InvalidCooldownOwner
    );
    // 3. check receiver usdu token account
    require!(
        ctx.accounts.receiver_usdu_token_account.key()
            == ctx.accounts.cooldown.underlying_token_account,
        VaultError::InvalidReceiverUsduTokenAccount
    );
    let vault_config = &mut ctx.accounts.vault_config;
    let usdu_amount = ctx.accounts.cooldown.underlying_token_amount;

    // 4. check usdu amount
    require!(usdu_amount > 0, VaultError::AmountMustBeGreaterThanZero);

    // 5. verify vault_silo_usdu_token_account has enough USDU to process withdrawal
    require!(
        ctx.accounts.vault_silo_usdu_token_account.amount >= usdu_amount,
        VaultError::InsufficientUsduInSilo
    );

    vault_config.total_cooldown_usdu_amount = vault_config
        .total_cooldown_usdu_amount
        .checked_sub(usdu_amount)
        .ok_or(VaultError::MathOverflow)?;

    // 6. set cooldown account underlying_token_amount to 0 to prevent reentrancy
    let cooldown = &mut ctx.accounts.cooldown;
    cooldown.cooldown_end = 0;
    cooldown.underlying_token_amount = 0;

    // 7. transfer usdu from vault_silo_usdu_token_account to receiver_usdu_token_account
    let config_bump = &[vault_config.bump];
    let config_seeds = &[&[VAULT_CONFIG_SEED, config_bump][..]];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.vault_silo_usdu_token_account.to_account_info(),
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
