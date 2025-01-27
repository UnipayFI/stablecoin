use anchor_lang::prelude::*;

use guardian::{Role, AccessRegistry, AccessRole};
use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};

use crate::error::VaultError;
use crate::events::BlacklistAdded;
use crate::state::{VaultConfig, BlacklistState};
use crate::utils::has_role_or_admin;
use crate::constants::{VAULT_BLACKLIST_SEED};

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct AdjustBlacklist<'info> {
    #[account(mut)]
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
            Role::GrandMaster.to_seed().as_slice()
        ],
        bump = grand_master.bump,
        seeds::program = guardian::id(),
    )]
    pub grand_master: Box<Account<'info, AccessRole>>,
    #[account(
        init_if_needed,
        payer = authority,
        space = BlacklistState::SIZE,
        seeds = [VAULT_BLACKLIST_SEED, user.as_ref()],
        bump,
    )]
    pub blacklist_state: Box<Account<'info, BlacklistState>>,
    pub system_program: Program<'info, System>,
}

pub fn process_adjust_blacklist(
    ctx: Context<AdjustBlacklist>,
    user: Pubkey,
    is_frozen_susdu: bool,
    is_frozen_usdu: bool
) -> Result<()> {
    // 1. check access role
    require!(
        has_role_or_admin(
            &ctx.accounts.vault_config, 
            &ctx.accounts.access_registry, 
            &ctx.accounts.grand_master.to_account_info(), 
            &ctx.accounts.authority.to_account_info(), 
            Role::GrandMaster
        )?,
        VaultError::UnauthorizedRole
    );
    if ctx.accounts.blacklist_state.is_initialized {
        return Err(error!(VaultError::BlacklistStateAlreadyInitialized));
    }
    ctx.accounts.blacklist_state.is_initialized = true;
    ctx.accounts.blacklist_state.bump = ctx.bumps.blacklist_state;
    ctx.accounts.blacklist_state.is_frozen_susdu = is_frozen_susdu;
    ctx.accounts.blacklist_state.is_frozen_usdu = is_frozen_usdu;
    ctx.accounts.blacklist_state.owner = user;
    emit!(BlacklistAdded {
        user,
        is_frozen_susdu,
        is_frozen_usdu,
    });
    Ok(())
}


