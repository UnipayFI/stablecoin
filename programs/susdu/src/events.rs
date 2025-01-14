use anchor_lang::prelude::*;

#[event]
pub struct SusduConfigInitialized {
    pub susdu_config: Pubkey,
    pub admin: Pubkey,
    pub access_registry: Pubkey,
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

