use anchor_lang::prelude::*;

use crate::error::VaultError;
use crate::events::CooldownAdjusted;
use crate::state::VaultConfig;
use crate::utils::has_role_or_admin;

use guardian::{Role, AccessRegistry, AccessRole};
use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};

#[derive(Accounts)]
pub struct AdjustCooldown<'info> {
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
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), authority.key().as_ref(), Role::VaultManager.to_seed().as_slice()],
        bump = vault_manager.bump,
        seeds::program = guardian::id(),
    )]
    pub vault_manager: Box<Account<'info, AccessRole>>,
    pub system_program: Program<'info, System>,
}

pub fn process_adjust_cooldown(ctx: Context<AdjustCooldown>, cooldown_duration: u64) -> Result<()> {
    require!(
        has_role_or_admin(
            &ctx.accounts.vault_config, 
            &ctx.accounts.access_registry, 
            &ctx.accounts.vault_manager, 
            &ctx.accounts.authority, 
            Role::VaultManager
        )?, 
        VaultError::UnauthorizedRole
    );
    let vault_config = &mut ctx.accounts.vault_config;
    vault_config.cooldown_duration = cooldown_duration;
    emit!(CooldownAdjusted {
        vault_config: ctx.accounts.vault_config.key(),
        cooldown_duration,
    });
    Ok(())
}
