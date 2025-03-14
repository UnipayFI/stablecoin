use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::constants::{
    VAULT_CONFIG_SEED, VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED, VAULT_STATE_SEED,
};
use crate::error::VaultError;
use crate::state::{VaultConfig, VaultState};

use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::utils::has_role;

use susdu::cpi::{accounts::MintSusdu, mint_susdu};
use susdu::program::Susdu;
use susdu::state::SusduConfig;
use susdu::SUSDU_CONFIG_SEED;

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
            &ctx.accounts.susdu_minter.to_account_info(),
            &ctx.accounts.vault_config.to_account_info(),
            Role::SusduMinter,
        )?,
        VaultError::UnauthorizedRole
    );

    // 2. check vault_stake_pool_usdu_token_account
    require!(
        ctx.accounts.vault_stake_pool_usdu_token_account.key()
            == ctx.accounts.vault_state.vault_stake_pool_usdu_token_account,
        VaultError::InvalidVaultStakePoolUsduTokenAccount
    );
    // 3. check usdu amount and initial deposit
    require!(usdu_amount > 0, VaultError::InvalidStakeUsduAmount);
    let vault_config = &mut ctx.accounts.vault_config;
    vault_config.check_initial_deposit(usdu_amount)?;

    // 4. check max deposit
    let max_assets = vault_config.max_deposit();
    require!(usdu_amount <= max_assets, VaultError::MaxDepositExceeded);

    require!(
        ctx.accounts.caller_usdu_token_account.amount >= usdu_amount,
        VaultError::InsufficientUsduBalance
    );

    // 5. calculate susdu amount(shares amount) to transfer to receiver_susdu_token_account
    let total_susdu_supply = ctx.accounts.susdu_config.total_supply;
    let susdu_amount = if !vault_config.has_initial_deposit {
        // Initial deposit, set initial shares amount
        usdu_amount
    } else {
        vault_config.preview_deposit(usdu_amount, total_susdu_supply)
    };
    require!(
        susdu_amount > 0,
        VaultError::InvalidPreviewDepositSusduAmount
    );

    // 6. update total_staked_usdu_supply
    vault_config.total_staked_usdu_supply = vault_config.total_staked_usdu_supply + usdu_amount;

    // 7. transfer usdu from caller to vault_pool_usdu_token_account
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.caller_usdu_token_account.to_account_info(),
                to: ctx
                    .accounts
                    .vault_stake_pool_usdu_token_account
                    .to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
            },
        ),
        usdu_amount,
        ctx.accounts.usdu_token.decimals,
    )?;

    // 8. mint susdu to receiver_susdu_token_account
    let config_bump = &[vault_config.bump];
    let config_seeds = &[&[VAULT_CONFIG_SEED, config_bump][..]];
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

    // 9. update initial deposit status
    if !vault_config.has_initial_deposit {
        vault_config.has_initial_deposit = true;
    }

    // 10. reload susdu_config to get updated total_supply
    let susdu_config = &ctx.accounts.susdu_config.to_account_info();
    let susdu_config_data = susdu_config.try_borrow_data()?;
    let updated_total_supply = SusduConfig::try_deserialize(&mut &susdu_config_data[..])?;

    // 11. check min shares with updated total_supply, reload susdu_config first
    ctx.accounts.susdu_config.reload()?;
    vault_config.check_min_shares(updated_total_supply.total_supply)?;
    Ok(())
}
