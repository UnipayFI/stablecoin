use anchor_lang::prelude::*;

#[event]
pub struct AdminTransferProposed {
    pub blacklist_hook_config: Pubkey,
    pub current_admin: Pubkey,
    pub proposed_admin: Pubkey,
}

#[event]
pub struct AdminTransferCompleted {
    pub blacklist_hook_config: Pubkey,
    pub previous_admin: Pubkey,
    pub new_admin: Pubkey,
}

#[event]
pub struct BlacklistAdded {
    pub user: Pubkey,
    pub blacklist_entry: Pubkey,
    pub blacklist_hook_config: Pubkey,
}

#[event]
pub struct BlacklistRemoved {
    pub user: Pubkey,
    pub blacklist_entry: Pubkey,
    pub blacklist_hook_config: Pubkey,
}
