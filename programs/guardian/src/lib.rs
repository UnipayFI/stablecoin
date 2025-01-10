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

declare_id!("5B6ex5JPZMv6t2tHzi4iz1tLb3y8JCqcPpefRqq68wGH");

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
}
