#[allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("2EfRohHd6CMfGPYTq16g4UYCD6f6vPzUpq1urgbwD4QB");

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

    pub fn propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
        process_propose_new_admin(ctx)
    }

    pub fn accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
        process_accept_admin_transfer(ctx)
    }
}
