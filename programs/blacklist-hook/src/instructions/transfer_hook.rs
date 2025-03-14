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

use crate::constants::{BLACKLIST_ENTRY_SEED, EXTRA_ACCOUNT_META_LIST_SEED};
use crate::error::BlacklistHookError;
use crate::utils::is_in_blacklist;

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
    #[account(
        seeds = [BLACKLIST_ENTRY_SEED.as_bytes(), source_token.owner.as_ref()],
        bump,
    )]
    /// CHECK: This is the source blacklist entry, may not exist
    pub source_blacklist_entry: UncheckedAccount<'info>,
    #[account(
        seeds = [BLACKLIST_ENTRY_SEED.as_bytes(), destination_token.owner.as_ref()],
        bump,
    )]
    /// CHECK: This is the destination blacklist entry, may not exist
    pub destination_blacklist_entry: UncheckedAccount<'info>,
}

pub fn process_transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
    let source_token_info = ctx.accounts.source_token.to_account_info();
    let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
    let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
    let account_with_extensions = account.get_extension_mut::<TransferHookAccount>()?;

    if !bool::from(account_with_extensions.transferring) {
        return err!(BlacklistHookError::IsNotTransferring);
    }

    if is_in_blacklist(
        &ctx.accounts.source_blacklist_entry.to_account_info(),
        &ctx.accounts.source_token.owner,
    )? {
        return err!(BlacklistHookError::SourceAddressBlacklisted);
    }

    if is_in_blacklist(
        &ctx.accounts.destination_blacklist_entry.to_account_info(),
        &ctx.accounts.destination_token.owner,
    )? {
        return err!(BlacklistHookError::DestinationAddressBlacklisted);
    }

    Ok(())
}
