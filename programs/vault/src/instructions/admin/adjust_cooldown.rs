use anchor_lang::prelude::*;

use crate::error::VaultError;
use crate::events::CooldownAdjusted;
use crate::state::VaultConfig;

#[derive(Accounts)]
pub struct AdjustCooldown<'info> {
    #[account(mut)]
    pub vault_config: Box<Account<'info, VaultConfig>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn process_adjust_cooldown(ctx: Context<AdjustCooldown>, cooldown_duration: u64) -> Result<()> {
    let vault_config = &mut ctx.accounts.vault_config;
    require!(ctx.accounts.authority.key() == vault_config.admin, VaultError::Unauthorized);
    vault_config.cooldown_duration = cooldown_duration;
    emit!(CooldownAdjusted {
        vault_config: ctx.accounts.vault_config.key(),
        cooldown_duration,
    });
    Ok(())
}
