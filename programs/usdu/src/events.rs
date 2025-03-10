use anchor_lang::prelude::*;

#[event]
pub struct UsduConfigInitialized {
    pub usdu_config: Pubkey,
    pub admin: Pubkey,
    pub access_registry: Pubkey,
}

#[event]
pub struct UsduTokenCreated {
    pub usdu_token: Pubkey,
    pub decimals: u8,
}

#[event]
pub struct UsduTokenMinted {
    pub usdu_token: Pubkey,
    pub amount: u64,
    pub receiver: Pubkey,
    pub receiver_token_account: Pubkey,
}

#[event]
pub struct UsduTokenRedeemed {
    pub usdu_token: Pubkey,
    pub amount: u64,
    pub caller: Pubkey,
    pub caller_token_account: Pubkey,
}

#[event]
pub struct AdminTransferProposed {
    pub usdu_config: Pubkey,
    pub current_admin: Pubkey,
    pub proposed_admin: Pubkey,
}

#[event]
pub struct AdminTransferCompleted {
    pub usdu_config: Pubkey,
    pub previous_admin: Pubkey,
    pub new_admin: Pubkey,
}
