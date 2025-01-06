use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, TokenAccount, Mint};
use anchor_spl::token_2022::{
    TransferChecked,
    transfer_checked,
};
use anchor_spl::associated_token::AssociatedToken;

use crate::state::{VaultConfig, VaultState};
use crate::error::VaultError;
use crate::constants::{
    VAULT_STATE_SEED,
    VAULT_CONFIG_SEED,
    VAULT_SLIO_USDU_TOKEN_ACCOUNT_SEED,
};

use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};

#[derive(Accounts)]
pub struct DistributeUsduReward<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
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
         associated_token::authority = caller,
         associated_token::token_program = token_program,
     )]
     pub caller_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
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
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), caller.key().as_ref(), Role::DistributeRewarder.to_seed().as_slice()],
        bump = distribute_rewarder.bump,
        seeds::program = guardian::id(),
    )]
    pub distribute_rewarder: Box<Account<'info, AccessRole>>,

    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_distribute_usdu_reward(
    ctx: Context<DistributeUsduReward>,
    usdu_amount: u64,
) -> Result<()> {
    require!(ctx.accounts.caller_usdu_token_account.amount >= usdu_amount, VaultError::InsufficientUsduBalance);
    let vault_config = &mut ctx.accounts.vault_config;
    require!(vault_config.get_unvested_amount() <= 0, VaultError::StillVesting);

    // update vault config
    vault_config.vesting_amount = usdu_amount + vault_config.get_unvested_amount();
    vault_config.last_distribution_timestamp = Clock::get()?.unix_timestamp as u64;
    vault_config.total_usdu_supply = vault_config.total_usdu_supply + usdu_amount;

    // transfer usdu to vault slio usdu token account
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.caller_usdu_token_account.to_account_info(),
                to: ctx.accounts.vault_slio_usdu_token_account.to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
            },
        ),
        usdu_amount,
        ctx.accounts.usdu_token.decimals,
    )?;
    Ok(())
}
