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

declare_id!("2J7qaFqKNUA2AexBxuRzpbwUXzJEdXUPTjRn487x1YFS");

#[program]
pub mod susdu {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
        process_init_config(ctx)
    }

    pub fn create_susdu(ctx: Context<CreateSusdu>, decimals: u8) -> Result<()> {
        process_create_susdu(ctx, decimals)
    }

    pub fn mint_susdu(ctx: Context<MintSusdu>, susdu_amount: u64) -> Result<()> {
        process_mint_susdu(ctx, susdu_amount)
    }

    pub fn redeem_susdu(ctx: Context<RedeemSusdu>, susdu_amount: u64) -> Result<()> {
        process_redeem_susdu(ctx, susdu_amount)
    }

    pub fn redistribute_susdu(ctx: Context<RedistributeSusdu>, receiver: Pubkey, amount: u64) -> Result<()> {
        process_redistribute_susdu(ctx, receiver, amount)
    }
}
