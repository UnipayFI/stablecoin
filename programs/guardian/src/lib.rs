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

declare_id!("H78gQvGwKs8skwuCyTvV53oNeUVcrY3mWrBpjhB9MfoJ");

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
