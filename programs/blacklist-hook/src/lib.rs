pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;
use anchor_lang::prelude::*;
use spl_transfer_hook_interface::instruction::TransferHookInstruction;

pub use constants::*;
pub use error::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

declare_id!("3L8uCaQSScHynZYFjHhYhrUFFguDUq8wjVe7T2NwXoR3");

#[program]
pub mod blacklist_hook {
    use super::*;

    pub fn initialize_extra_account_meta(ctx: Context<InitializeExtraAccountMeta>) -> Result<()> {
        process_initialize_extra_account_meta(ctx)
    }

    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        msg!("transfer_hook");
        process_transfer_hook(ctx, amount)
    }

    pub fn propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
        process_propose_new_admin(ctx)
    }

    pub fn accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
        process_accept_admin_transfer(ctx)
    }

    pub fn add_to_blacklist(ctx: Context<AddToBlacklist>) -> Result<()> {
        process_add_to_blacklist(ctx)
    }

    pub fn remove_from_blacklist(ctx: Context<RemoveFromBlacklist>) -> Result<()> {
        process_remove_from_blacklist(ctx)
    }

    pub fn fallback<'info>(
        program_id: &Pubkey,
        accounts: &'info [AccountInfo<'info>],
        instruction_data: &[u8],
    ) -> Result<()> {
        msg!("fallback");
        let instruction = TransferHookInstruction::unpack(instruction_data)?;
        match instruction {
            TransferHookInstruction::Execute { amount } => {
                let amount_le_bytes = amount.to_le_bytes();
                __private::__global::transfer_hook(program_id, accounts, &amount_le_bytes)
            }
            _ => return Err(ProgramError::InvalidInstructionData.into()),
        }
    }
}
