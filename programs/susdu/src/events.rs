use anchor_lang::prelude::*;

#[event]
pub struct SusduConfigInitialized {
    pub susdu_config: Pubkey,
    pub admin: Pubkey,
    pub access_registry: Pubkey,
    pub blacklist_hook_program_id: Pubkey,
}

#[event]
pub struct SusduTokenCreated {
    pub susdu_token: Pubkey,
    pub decimals: u8,
}

#[event]
pub struct SusduTokenMinted {
    pub susdu_token: Pubkey,
    pub amount: u64,
    pub receiver: Pubkey,
    pub receiver_token_account: Pubkey,
}

#[event]
pub struct SusduTokenRedeemed {
    pub susdu_token: Pubkey,
    pub amount: u64,
    pub caller: Pubkey,
    pub caller_token_account: Pubkey,
}

#[event]
pub struct SusduTokenRedistributed {
    pub susdu_token: Pubkey,
    pub amount: u64,
    pub from: Pubkey,
    pub to: Pubkey,
    pub is_burned: bool,
}

#[event]
pub struct AdminTransferProposed {
    pub susdu_config: Pubkey,
    pub current_admin: Pubkey,
    pub proposed_admin: Pubkey,
}

#[event]
pub struct AdminTransferCompleted {
    pub susdu_config: Pubkey,
    pub previous_admin: Pubkey,
    pub new_admin: Pubkey,
}

#[event]
pub struct TransferHookUpdated {
    pub susdu_config: Pubkey,
    pub old_transfer_hook_program_id: Pubkey,
    pub new_transfer_hook_program_id: Pubkey,
}
