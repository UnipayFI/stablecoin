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

