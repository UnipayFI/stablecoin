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

declare_id!("BQVfERSxGzQJ2fzH1LkSqhBqKmHfED3FtvdCr1Un3pqm");

#[program]
pub mod susdu {
    use super::*;

    pub fn init_config(ctx: Context<InitConfig>, blacklist_hook_program_id: Pubkey) -> Result<()> {
        process_init_config(ctx, blacklist_hook_program_id)
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

    pub fn redistribute_susdu(
        ctx: Context<RedistributeSusdu>,
        receiver: Pubkey,
        amount: u64,
    ) -> Result<()> {
        process_redistribute_susdu(ctx, receiver, amount)
    }

    pub fn update_transfer_hook(
        ctx: Context<UpdateTransferHook>,
        transfer_hook_program_id: Pubkey,
    ) -> Result<()> {
        process_update_transfer_hook(ctx, transfer_hook_program_id)
    }

    pub fn propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
        process_propose_new_admin(ctx)
    }

    pub fn accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
        process_accept_admin_transfer(ctx)
    }
}
