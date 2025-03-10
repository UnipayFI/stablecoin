use anchor_lang::prelude::*;

use crate::error::VaultError;
use crate::events::CooldownAdjusted;
use crate::state::VaultConfig;
use crate::utils::has_role_or_admin;

use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};
use guardian::{AccessRegistry, AccessRole, Role};

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
        seeds = [ACCESS_ROLE_SEED, access_registry.key().as_ref(), authority.key().as_ref(), Role::VaultAdmin.to_seed().as_slice()],
        bump = vault_admin.bump,
        seeds::program = guardian::id(),
    )]
    pub vault_admin: Box<Account<'info, AccessRole>>,
    pub system_program: Program<'info, System>,
}

pub fn process_adjust_cooldown(ctx: Context<AdjustCooldown>, cooldown_duration: u64) -> Result<()> {
    // Check if the caller has VaultAdmin role or is an admin
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

    // Validate cooldown duration is within allowed range
    require!(
        cooldown_duration >= crate::constants::MIN_COOLDOWN_DURATION,
        VaultError::CooldownDurationTooShort
    );
    require!(
        cooldown_duration <= crate::constants::MAX_COOLDOWN_DURATION,
        VaultError::CooldownDurationTooLong
    );

    let vault_config = &mut ctx.accounts.vault_config;

    // Store the old cooldown duration for comparison
    let old_cooldown_duration = vault_config.cooldown_duration;

    // Update the cooldown duration
    vault_config.cooldown_duration = cooldown_duration;

    // Only emit the event if the cooldown duration has actually changed
    // This prevents misleading information and unnecessary event emissions
    if old_cooldown_duration != cooldown_duration {
        emit!(CooldownAdjusted {
            vault_config: ctx.accounts.vault_config.key(),
            cooldown_duration,
        });
    }

    Ok(())
}
