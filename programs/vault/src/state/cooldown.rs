use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct Cooldown {
    pub cooldown_end: u64,
    pub underlying_token_account: Pubkey,
    pub underlying_token_mint: Pubkey,
    pub underlying_token_amount: u64,
    pub owner: Pubkey,
    pub bump: u8,
    pub is_initialized: bool,
}

impl Cooldown {
    pub const SIZE: usize = 8 + Self::INIT_SPACE;

    pub fn is_cooldown_active(&self) -> bool {
        self.cooldown_end > Clock::get().unwrap().unix_timestamp as u64
    }

}
