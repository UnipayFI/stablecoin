use anchor_lang::prelude::*;

use crate::constants::BLACKLIST_HOOK_CONFIG;
use crate::error::BlacklistHookError;
use crate::events::{BlacklistAdded, BlacklistRemoved};
use crate::state::BlacklistHookConfig;

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

    pub system_program: Program<'info, System>,
}

pub fn process_add_to_blacklist(ctx: Context<AddToBlacklist>) -> Result<()> {
    let blacklist_hook_config = &mut ctx.accounts.blacklist_hook_config;
    let user_key = ctx.accounts.user.key();

    // Check if user is already in blacklist
    if blacklist_hook_config.blacklist.contains(&user_key) {
        return err!(BlacklistHookError::UserAlreadyInBlacklist);
    }

    // Add user to blacklist
    blacklist_hook_config.blacklist.push(user_key);

    // Emit event
    emit!(BlacklistAdded {
        user: user_key,
        blacklist_hook_config: blacklist_hook_config.key(),
    });

    Ok(())
}

pub fn process_remove_from_blacklist(ctx: Context<RemoveFromBlacklist>) -> Result<()> {
    let blacklist_hook_config = &mut ctx.accounts.blacklist_hook_config;
    let user_key = ctx.accounts.user.key();

    // Find user in blacklist
    let position = blacklist_hook_config
        .blacklist
        .iter()
        .position(|&x| x == user_key);

    // Check if user is in blacklist
    if let Some(index) = position {
        // Remove user from blacklist
        blacklist_hook_config.blacklist.remove(index);

        // Emit event
        emit!(BlacklistRemoved {
            user: user_key,
            blacklist_hook_config: blacklist_hook_config.key(),
        });

        Ok(())
    } else {
        err!(BlacklistHookError::UserNotInBlacklist)
    }
}
