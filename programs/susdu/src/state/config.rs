use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct SusduConfig {
    pub admin: Pubkey,
    pub access_registry: Pubkey,
    pub bump: u8,
    pub is_initialized: bool,

    pub susdu_token: Pubkey,
    pub susdu_token_bump: u8,
    pub is_susdu_token_initialized: bool,

    pub total_supply: u64,
}

impl SusduConfig {
    pub const SIZE: usize = 8 + SusduConfig::INIT_SPACE;
}
