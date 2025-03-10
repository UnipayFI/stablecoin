use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use guardian::state::{AccessRegistry, AccessRole, Role};

use crate::constants::{
    VAULT_CONFIG_SEED, VAULT_COOLDOWN_SEED, VAULT_SILO_USDU_TOKEN_ACCOUNT_SEED,
    VAULT_STAKE_POOL_USDU_TOKEN_ACCOUNT_SEED, VAULT_STATE_SEED, VAULT_SUSDU_TOKEN_ACCOUNT_SEED,
};
use crate::error::VaultError;
use crate::state::{Cooldown, VaultConfig, VaultState};

use susdu::cpi::{accounts::RedeemSusdu, redeem_susdu};
use susdu::program::Susdu;
use susdu::state::SusduConfig;
use susdu::SUSDU_CONFIG_SEED;

use guardian::utils::has_role;

#[derive(Accounts)]
pub struct UnstakeSusdu<'info> {
    #[account(mut)]
    pub caller: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = susdu_token,
        associated_token::authority = caller,
        associated_token::token_program = token_program,
    )]
    pub caller_susdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: no need to checked
    pub receiver: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = usdu_token,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
        seeds::program = susdu::id(),
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,
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
        seeds = [VAULT_SUSDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_susdu_token_account_bump,
        token::mint = susdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_susdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = caller,
        space = Cooldown::SIZE,
        seeds = [VAULT_COOLDOWN_SEED, usdu_token.key().as_ref(), receiver.key().as_ref(), caller.key().as_ref()],
        bump,
        constraint = cooldown.is_initialized == false || cooldown.owner == caller.key() @ VaultError::InvalidCooldownOwner,
    )]
    pub cooldown: Box<Account<'info, Cooldown>>,
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), vault_config.key().as_ref(), Role::SusduRedeemer.to_seed().as_slice()],
        bump = susdu_redeemer.bump,
        seeds::program = guardian::id(),
    )]
    pub susdu_redeemer: Box<Account<'info, AccessRole>>,

    #[account(mut)]
    pub susdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub usdu_token: Box<InterfaceAccount<'info, Mint>>,
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
        mut,
        seeds = [VAULT_SILO_USDU_TOKEN_ACCOUNT_SEED],
        bump = vault_state.vault_silo_usdu_token_account_bump,
        token::mint = usdu_token,
        token::authority = vault_config,
        token::token_program = token_program,
    )]
    pub vault_silo_usdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    pub susdu_program: Program<'info, Susdu>,
    pub token_program: Program<'info, Token2022>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub(crate) fn process_unstake_susdu(ctx: Context<UnstakeSusdu>, susdu_amount: u64) -> Result<()> {
    // 1. check caller has the role
    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.susdu_redeemer.to_account_info(),
            &ctx.accounts.vault_config.to_account_info(),
            Role::SusduRedeemer,
        )?,
        VaultError::UnauthorizedRole
    );

    require!(
        ctx.accounts.usdu_token.key() == ctx.accounts.vault_config.usdu,
        VaultError::InvalidUsduToken
    );
    require!(
        ctx.accounts.susdu_token.key() == ctx.accounts.vault_config.susdu,
        VaultError::InvalidSusduToken
    );

    // 2. check vault stake pool usdu token account
    require!(
        ctx.accounts
            .vault_state
            .vault_stake_pool_usdu_token_account
            .key()
            == ctx.accounts.vault_state.vault_stake_pool_usdu_token_account,
        VaultError::InvalidVaultStakePoolUsduTokenAccount
    );
    // 3. check vault slio usdu token account
    require!(
        ctx.accounts.vault_state.vault_silo_usdu_token_account.key()
            == ctx.accounts.vault_silo_usdu_token_account.key(),
        VaultError::InvalidVaultSiloUsduTokenAccount
    );
    // 4. check vault susdu token account
    require!(
        ctx.accounts.vault_state.vault_susdu_token_account.key()
            == ctx.accounts.vault_susdu_token_account.key(),
        VaultError::InvalidVaultSusduTokenAccount
    );
    // 5. check susdu amount
    require!(susdu_amount > 0, VaultError::InvalidUnstakeSusduAmount);
    let total_susdu_amount = ctx.accounts.susdu_config.total_supply;
    let vault_config = &mut ctx.accounts.vault_config;
    // 6. check caller has enough susdu
    require!(
        ctx.accounts.caller_susdu_token_account.amount >= susdu_amount,
        VaultError::InsufficientSusdu
    );
    let usdu_amount = vault_config.preview_redeem(susdu_amount, total_susdu_amount);
    // 7. check preview redeem usdu amount
    require!(usdu_amount > 0, VaultError::InvalidPreviewRedeemUsduAmount);
    // 8. check total usdu supply
    require!(
        vault_config.total_staked_usdu_supply >= usdu_amount,
        VaultError::InsufficientUsduSupply
    );

    vault_config.total_staked_usdu_supply = vault_config.total_staked_usdu_supply - usdu_amount;
    // 9. check caller_susdu_token_account has enough susdu
    require!(
        ctx.accounts.caller_susdu_token_account.amount >= susdu_amount,
        VaultError::InsufficientSusdu
    );

    // 10. check cooldown is initialized
    if !ctx.accounts.cooldown.is_initialized {
        let cooldown = Cooldown {
            is_initialized: true,
            cooldown_end: Clock::get().unwrap().unix_timestamp as u64
                + vault_config.cooldown_duration,
            underlying_token_account: ctx
                .accounts
                .receiver_usdu_token_account
                .to_account_info()
                .key(),
            underlying_token_mint: ctx.accounts.usdu_token.to_account_info().key(),
            underlying_token_amount: usdu_amount,
            owner: ctx.accounts.caller.key(),
            bump: ctx.bumps.cooldown,
        };
        ctx.accounts.cooldown.set_inner(cooldown);
    } else {
        require!(
            ctx.accounts.cooldown.owner == ctx.accounts.caller.key(),
            VaultError::InvalidCooldownOwner
        );
        let cooldown = &mut ctx.accounts.cooldown;

        // Consistent with Ethena protocol, reset cooldown period for each unstake
        cooldown.cooldown_end =
            Clock::get().unwrap().unix_timestamp as u64 + vault_config.cooldown_duration;
        cooldown.underlying_token_amount += usdu_amount;
    };
    // 11. transfer susdu from caller to vault susdu token account
    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.caller_susdu_token_account.to_account_info(),
                to: ctx.accounts.vault_susdu_token_account.to_account_info(),
                mint: ctx.accounts.susdu_token.to_account_info(),
                authority: ctx.accounts.caller.to_account_info(),
            },
        ),
        susdu_amount,
        ctx.accounts.susdu_token.decimals,
    )?;

    // 12. redeem susdu from vault susdu token account
    let config_bump = &[vault_config.bump];
    let config_seeds = &[&[VAULT_CONFIG_SEED, config_bump][..]];
    redeem_susdu(
        CpiContext::new_with_signer(
            ctx.accounts.susdu_program.to_account_info(),
            RedeemSusdu {
                caller: vault_config.to_account_info(),
                access_registry: ctx.accounts.access_registry.to_account_info(),
                access_role: ctx.accounts.susdu_redeemer.to_account_info(),
                susdu_config: ctx.accounts.susdu_config.to_account_info(),
                susdu_token: ctx.accounts.susdu_token.to_account_info(),
                caller_token_account: ctx.accounts.vault_susdu_token_account.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            config_seeds,
        ),
        susdu_amount,
    )?;

    // 13. transfer usdu from vault_stake_pool_usdu_token_account to vault_silo_usdu_token_account
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx
                    .accounts
                    .vault_stake_pool_usdu_token_account
                    .to_account_info(),
                to: ctx.accounts.vault_silo_usdu_token_account.to_account_info(),
                authority: vault_config.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
            },
            config_seeds,
        ),
        usdu_amount,
        ctx.accounts.usdu_token.decimals,
    )?;

    // 14. check min shares
    vault_config.check_min_shares(ctx.accounts.susdu_config.total_supply)?;
    Ok(())
}
