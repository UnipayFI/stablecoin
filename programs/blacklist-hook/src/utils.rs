use anchor_lang::prelude::*;

use crate::constants::BLACKLIST_ENTRY_SEED;
use crate::state::BlacklistEntry;

pub fn is_in_blacklist<'info>(
    blacklist_entry: &AccountInfo<'info>,
    owner: &Pubkey,
) -> Result<bool> {
    // if not init this account, return false
    if blacklist_entry.owner != &crate::id() || blacklist_entry.data_len() == 0 {
        return Ok(false);
    }

    let account_data = &mut blacklist_entry.try_borrow_data()?;
    let blacklist_entry_account: BlacklistEntry =
        BlacklistEntry::try_deserialize(&mut &account_data[..])?;

    // Verify the blacklist_entry account address is derived correctly
    let (expected_blacklist_entry, _) = Pubkey::find_program_address(
        &[BLACKLIST_ENTRY_SEED.as_bytes(), owner.as_ref()],
        &crate::id(),
    );

    if blacklist_entry.key() != expected_blacklist_entry {
        return Ok(false);
    }

    if blacklist_entry_account.owner != *owner {
        return Ok(false);
    }

    Ok(blacklist_entry_account.is_active)
}
