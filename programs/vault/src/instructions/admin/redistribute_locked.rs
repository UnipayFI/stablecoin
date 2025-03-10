use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use susdu::cpi::{accounts::RedistributeSusdu, redistribute_susdu};
use susdu::program::Susdu;
use susdu::state::SusduConfig;
use susdu::SUSDU_CONFIG_SEED;

use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use guardian::state::{AccessRegistry, AccessRole, Role};

use blacklist_hook::constants::BLACKLIST_HOOK_CONFIG;
use blacklist_hook::program::BlacklistHook;
use blacklist_hook::state::BlacklistHookConfig;

use crate::constants::VAULT_CONFIG_SEED;
use crate::error::VaultError;
use crate::events::LockedSusduRedistributed;
use crate::state::VaultConfig;
use crate::utils::has_role_or_admin;

#[derive(Accounts)]
pub struct RedistributeLocked<'info> {
    #[account(
        mut,
        seeds = [VAULT_CONFIG_SEED],
        bump = vault_config.bump,
    )]
    pub vault_config: Box<Account<'info, VaultConfig>>,
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
            Role::VaultAdmin.to_seed().as_slice(),
        ],
        bump = vault_admin.bump,
        seeds::program = guardian::id(),
    )]
    pub vault_admin: Box<Account<'info, AccessRole>>,
    #[account(
        seeds = [
            ACCESS_ROLE_SEED,
            access_registry.key().as_ref(),
            vault_config.key().as_ref(),
            Role::SusduDistributor.to_seed().as_slice(),
        ],
        seeds::program = guardian::id(),
        bump = susdu_redistributor.bump,
    )]
    pub susdu_redistributor: Box<Account<'info, AccessRole>>,
    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
        seeds::program = susdu::id(),
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,
    #[account(mut)]
    pub susdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub locked_susdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: will be checked in the instruction
    #[account(mut)]
    pub receiver_susdu_token_account: UncheckedAccount<'info>,
    #[account(
        seeds = [BLACKLIST_HOOK_CONFIG.as_bytes()],
        seeds::program = blacklist_hook::id(),
        bump = blacklist_hook_config.bump,
    )]
    pub blacklist_hook_config: Box<Account<'info, BlacklistHookConfig>>,

    pub blacklist_hook_program: Program<'info, BlacklistHook>,
    pub susdu_program: Program<'info, Susdu>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_redistribute_locked(
    ctx: Context<RedistributeLocked>,
    receiver: Pubkey,
) -> Result<()> {
    // 1. check user must have manager role
    require!(
        has_role_or_admin(
            &ctx.accounts.vault_config,
            &ctx.accounts.access_registry,
            &ctx.accounts.vault_admin.to_account_info(),
            &ctx.accounts.authority.to_account_info(),
            Role::VaultAdmin
        )?,
        VaultError::UnauthorizedRole
    );

    require!(
        ctx.accounts.susdu_token.key() == ctx.accounts.vault_config.susdu,
        VaultError::InvalidSusduToken
    );

    // 2. check user must be in blacklist
    require!(
        ctx.accounts
            .blacklist_hook_config
            .blacklist
            .contains(&ctx.accounts.locked_susdu_token_account.owner),
        VaultError::NotBlacklistAccount
    );

    // 3. check locked_susdu_token_account amount is greater than 0
    let locked_susdu_token_account_amount = ctx.accounts.locked_susdu_token_account.amount;
    require!(
        locked_susdu_token_account_amount > 0,
        VaultError::InvalidLockedSusduTokenAccountAmount
    );

    let config_bump = &[ctx.accounts.vault_config.bump];
    let signer_seeds = &[&[VAULT_CONFIG_SEED, config_bump][..]];
    redistribute_susdu(
        CpiContext::new_with_signer(
            ctx.accounts.susdu_program.to_account_info(),
            RedistributeSusdu {
                authority: ctx.accounts.vault_config.to_account_info(),
                susdu_config: ctx.accounts.susdu_config.to_account_info(),
                susdu_token: ctx.accounts.susdu_token.to_account_info(),
                locked_susdu_token_account: ctx
                    .accounts
                    .locked_susdu_token_account
                    .to_account_info(),
                receiver_susdu_token_account: ctx
                    .accounts
                    .receiver_susdu_token_account
                    .to_account_info(),
                access_registry: ctx.accounts.access_registry.to_account_info(),
                access_role: ctx.accounts.susdu_redistributor.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            signer_seeds,
        ),
        receiver,
        locked_susdu_token_account_amount,
    )?;

    emit!(LockedSusduRedistributed {
        from: ctx.accounts.locked_susdu_token_account.owner,
        to: receiver,
        amount: locked_susdu_token_account_amount,
        is_burned: false,
    });
    Ok(())
}
