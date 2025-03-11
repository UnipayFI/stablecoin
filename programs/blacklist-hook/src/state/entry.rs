use anchor_lang::prelude::*;

#[account]
pub struct BlacklistEntry {
    pub owner: Pubkey,
    pub is_active: bool,
}

impl BlacklistEntry {
    pub const SIZE: usize = 8 + std::mem::size_of::<Self>();
}
