use anchor_lang::prelude::*;

#[account]
pub struct BlacklistHookConfig {
    pub admin: Pubkey,
    pub pending_admin: Pubkey,
    pub bump: u8,
    pub is_initialized: bool,
}

impl BlacklistHookConfig {
    pub const SIZE: usize = 8 + std::mem::size_of::<Self>();
}
