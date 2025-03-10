#[allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use events::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

declare_id!("69S9GEFC4vimP2PnMrVFzpAZeD6u2EUi24PUWiYF8wtt");

#[program]
pub mod guardian {
    use super::*;

    pub fn init_access_registry(ctx: Context<InitAccessRegistry>) -> Result<()> {
        process_init_access_registry(ctx)
    }

    pub fn assign_role(ctx: Context<AssignRole>, role: Role) -> Result<()> {
        process_assign_role(ctx, role)
    }

    pub fn revoke_role(ctx: Context<RevokeRole>) -> Result<()> {
        process_revoke_role(ctx)
    }

    pub fn propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
        process_propose_new_admin(ctx)
    }

    pub fn accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
        process_accept_admin_transfer(ctx)
    }
}
