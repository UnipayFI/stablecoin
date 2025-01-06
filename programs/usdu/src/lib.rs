#[allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("BCwUVofFwsXLBN7LXiFWXbT2a5hhk2we7FN8C7zAosE4");

#[program]
pub mod usdu {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
        process_init_config(ctx)
    }

    pub fn create_usdu(ctx: Context<CreateUsdu>, decimals: u8) -> Result<()> {
        process_create_usdu(ctx, decimals)
    }

    pub fn mint_usdu(ctx: Context<MintUsdu>, usdu_amount: u64) -> Result<()> {
        process_mint_usdu(ctx, usdu_amount)
    }

    pub fn redeem_usdu(ctx: Context<RedeemUsdu>, usdu_amount: u64) -> Result<()> {
        process_redeem_usdu(ctx, usdu_amount)
    }
}
