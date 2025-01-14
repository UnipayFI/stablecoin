use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct BlacklistState {
    pub owner: Pubkey,
    pub bump: u8,
    pub is_initialized: bool,
    pub is_frozen_usdu: bool,
    pub is_frozen_susdu: bool,
}

impl BlacklistState {
    pub const SIZE: usize = 8 + Self::INIT_SPACE;
}
