use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use crate::state::BlacklistState;

pub fn is_blacklisted<'info>(
    blacklist_state: &AccountInfo<'info>,
) -> Result<bool> {
    // if not init this account, return false
    if blacklist_state.owner == &system_program::id() && blacklist_state.lamports() == 0 {
        return Ok(false);
    }
    
    // deserialize the account
    let blacklist_state_account = BlacklistState::try_deserialize(&mut &blacklist_state.data.borrow()[..])?;
    Ok(blacklist_state_account.is_initialized && (blacklist_state_account.is_frozen_susdu || blacklist_state_account.is_frozen_usdu))
}
