use anchor_lang::prelude::*;

use crate::constants::{BLACKLIST_ENTRY_SEED, BLACKLIST_HOOK_CONFIG};
use crate::error::BlacklistHookError;
use crate::events::{BlacklistAdded, BlacklistRemoved};
use crate::state::{BlacklistEntry, BlacklistHookConfig};

#[derive(Accounts)]
pub struct AddToBlacklist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: This is the user to be added to the blacklist
    pub user: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [BLACKLIST_HOOK_CONFIG.as_bytes()],
        bump = blacklist_hook_config.bump,
        constraint = blacklist_hook_config.admin == admin.key() @ BlacklistHookError::OnlyAdminCanModifyBlacklist,
    )]
    pub blacklist_hook_config: Box<Account<'info, BlacklistHookConfig>>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [BLACKLIST_ENTRY_SEED.as_bytes(), user.key().as_ref()],
        space = BlacklistEntry::SIZE,
        bump,
    )]
    pub blacklist_entry: Box<Account<'info, BlacklistEntry>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RemoveFromBlacklist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: This is the user to be removed from the blacklist
    pub user: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [BLACKLIST_HOOK_CONFIG.as_bytes()],
        bump = blacklist_hook_config.bump,
        constraint = blacklist_hook_config.admin == admin.key() @ BlacklistHookError::OnlyAdminCanModifyBlacklist,
    )]
    pub blacklist_hook_config: Box<Account<'info, BlacklistHookConfig>>,
    #[account(
        mut,
        seeds = [BLACKLIST_ENTRY_SEED.as_bytes(), user.key().as_ref()],
        bump,
        close = admin,
    )]
    pub blacklist_entry: Box<Account<'info, BlacklistEntry>>,
    pub system_program: Program<'info, System>,
}

pub fn process_add_to_blacklist(ctx: Context<AddToBlacklist>) -> Result<()> {
    let blacklist_hook_config = &mut ctx.accounts.blacklist_hook_config;
    let blacklist_entry = &mut ctx.accounts.blacklist_entry;
    let user_key = ctx.accounts.user.key();
    require!(
        !blacklist_entry.is_active,
        BlacklistHookError::BlacklistEntryAlreadyExists
    );

    blacklist_entry.is_active = true;
    blacklist_entry.owner = user_key;

    emit!(BlacklistAdded {
        user: user_key,
        blacklist_entry: blacklist_entry.key(),
        blacklist_hook_config: blacklist_hook_config.key(),
    });

    Ok(())
}

pub fn process_remove_from_blacklist(ctx: Context<RemoveFromBlacklist>) -> Result<()> {
    let blacklist_entry = &mut ctx.accounts.blacklist_entry;
    blacklist_entry.is_active = false;

    emit!(BlacklistRemoved {
        user: ctx.accounts.user.key(),
        blacklist_entry: ctx.accounts.blacklist_entry.key(),
        blacklist_hook_config: ctx.accounts.blacklist_hook_config.key(),
    });

    Ok(())
}
