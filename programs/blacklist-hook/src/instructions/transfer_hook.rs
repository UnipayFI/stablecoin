use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount, BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{Mint, TokenAccount},
};

use std::cell::RefMut;

use crate::constants::{BLACKLIST_HOOK_CONFIG, EXTRA_ACCOUNT_META_LIST_SEED};
use crate::error::BlacklistHookError;
use crate::state::BlacklistHookConfig;

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::authority = owner
    )]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(token::mint = mint)]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: This is the owner of the source token
    pub owner: UncheckedAccount<'info>,
    #[account(
        seeds = [EXTRA_ACCOUNT_META_LIST_SEED, mint.key().as_ref()],
        bump
    )]
    /// CHECK: This is the extra account meta list
    pub extra_account_meta_list: AccountInfo<'info>,
    #[account(seeds = [BLACKLIST_HOOK_CONFIG.as_bytes()], bump)]
    pub blacklist_hook_config: Box<Account<'info, BlacklistHookConfig>>,
}

pub fn process_transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
    let source_token_info = ctx.accounts.source_token.to_account_info();
    let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
    let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
    let account_with_extensions = account.get_extension_mut::<TransferHookAccount>()?;

    if !bool::from(account_with_extensions.transferring) {
        return err!(BlacklistHookError::IsNotTransferring);
    }

    let blacklist = &ctx.accounts.blacklist_hook_config.blacklist;
    if is_in_blacklist(blacklist, &ctx.accounts.owner.key()) {
        panic!("Operation not allowed");
    }
    if is_in_blacklist(blacklist, &ctx.accounts.destination_token.owner) {
        panic!("Operation not allowed");
    }

    Ok(())
}

fn is_in_blacklist(blacklist: &[Pubkey], key: &Pubkey) -> bool {
    blacklist.contains(key)
}
