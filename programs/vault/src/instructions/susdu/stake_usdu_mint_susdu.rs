use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Token2022, TokenAccount, Mint};
use anchor_spl::token_2022::{
    TransferChecked,
    transfer_checked,
};
use anchor_spl::associated_token::AssociatedToken;

use crate::utils::is_blacklisted;
use crate::state::{VaultState, VaultConfig};
use crate::constants::{
    VAULT_STATE_SEED,
    VAULT_CONFIG_SEED,
    VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED,
};
use crate::error::VaultError;

use guardian::utils::has_role;
use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};

use susdu::program::Susdu;
use susdu::state::SusduConfig;
use susdu::SUSDU_CONFIG_SEED;
use susdu::cpi::{mint_susdu, accounts::MintSusdu};

#[derive(Accounts)]
pub struct StakeUsduMintSusdu<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
    /// CHECK: no need to checked
    pub receiver: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = susdu_token,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_susdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = usdu_token,
        associated_token::authority = caller,
        associated_token::token_program = token_program,
    )]
    pub caller_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        mut,
        seeds = [VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_stake_pool_usdu_token_account_bump,
        token::mint = usdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_stake_pool_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), vault_config.key().as_ref(), Role::SusduMinter.to_seed().as_slice()],
        bump = susdu_minter.bump,
        seeds::program = guardian::id(),
    )]
    pub susdu_minter: Box<Account<'info, AccessRole>>,

    #[account(mut)]
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub susdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        seeds = [VAULT_STATE_SEED],
        bump = vault_state.bump,
        has_one = vault_stake_pool_usdu_token_account,
    )]
    pub vault_state: Box<Account<'info, VaultState>>,
    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
        seeds::program = susdu::id(),
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,
    /// CHECK: will be checked in the process function
    pub blacklist_state: UncheckedAccount<'info>,

    pub susdu_program: Program<'info, Susdu>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_stake_usdu_mint_susdu(
    ctx: Context<StakeUsduMintSusdu>,
    usdu_amount: u64,
) -> Result<()> {
    // 1. check access role
    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.susdu_minter,
            &ctx.accounts.vault_config.to_account_info(),
            Role::SusduMinter,
        )?,
        VaultError::UnauthorizedRole
    );
    // 2. check blacklist
    require!(
        !is_blacklisted(&ctx.accounts.blacklist_state.to_account_info())?,
        VaultError::BlacklistAccount
    );
    // 3. check vault_stake_pool_usdu_token_account
    require!(
        ctx.accounts.vault_stake_pool_usdu_token_account.key() == ctx.accounts.vault_state.vault_stake_pool_usdu_token_account,
         VaultError::InvalidVaultStakePoolUsduTokenAccount
    );
    // 4. check usdu amount
    require!(usdu_amount > 0, VaultError::InvalidStakeUsduAmount);
    let total_susdu_supply = ctx.accounts.susdu_config.total_supply;
    // 5. check max deposit
    let max_assets = ctx.accounts.vault_config.max_deposit();
    require!(usdu_amount <= max_assets, VaultError::MaxDepositExceeded);
    // 6. calculate susdu amount(shares amount) to transfer to receiver_susdu_token_account
    let susdu_amount = ctx.accounts.vault_config.preview_deposit(
        usdu_amount,
        total_susdu_supply,
    );
    require!(susdu_amount > 0, VaultError::InvalidPreviewDepositSusduAmount);

    // 7. update total_usdu_supply
    let vault_config = &mut ctx.accounts.vault_config;
    vault_config.total_usdu_supply = vault_config.total_usdu_supply + usdu_amount;
    // 8. transfer usdu from caller to vault_pool_usdu_token_account
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.caller_usdu_token_account.to_account_info(),
                to: ctx.accounts.vault_stake_pool_usdu_token_account.to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
            },
        ),
        usdu_amount,
        ctx.accounts.usdu_token.decimals,
    )?;
    // 9. mint susdu to receiver_susdu_token_account
    let config_bump = &[vault_config.bump];
    let config_seeds = &[
        &[
            VAULT_CONFIG_SEED,
            config_bump,
        ][..],
    ];
    mint_susdu(
        CpiContext::new_with_signer(
            ctx.accounts.susdu_program.to_account_info(),
            MintSusdu {
                authority: vault_config.to_account_info(),
                access_registry: ctx.accounts.access_registry.to_account_info(),
                access_role: ctx.accounts.susdu_minter.to_account_info(),
                susdu_config: ctx.accounts.susdu_config.to_account_info(),
                susdu_token: ctx.accounts.susdu_token.to_account_info(),
                receiver: ctx.accounts.receiver.to_account_info(),
                receiver_token_account: ctx.accounts.receiver_susdu_token_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
            },
            config_seeds,
        ),
        susdu_amount,
    )?;

    // 10. check min shares
    vault_config.check_min_shares(ctx.accounts.susdu_config.total_supply)?;
    Ok(())
}
