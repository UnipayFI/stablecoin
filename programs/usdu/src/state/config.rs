use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct UsduConfig {
    pub admin: Pubkey,
    pub access_registry: Pubkey,
    pub bump: u8,
    pub is_initialized: bool,

    pub usdu_token: Pubkey,
    pub usdu_token_bump: u8,
    pub is_usdu_token_initialized: bool,

    pub total_supply: u64,
}

impl UsduConfig {
    pub const SIZE: usize = 8 + UsduConfig::INIT_SPACE;
}
