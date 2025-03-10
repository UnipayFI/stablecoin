use anchor_lang::prelude::*;

use crate::state::VaultConfig;

use guardian::utils::has_role;
use guardian::{AccessRegistry, Role};

pub fn has_role_or_admin<'info>(
    vault_config: &Account<VaultConfig>,
    access_registry: &Account<AccessRegistry>,
    access_role: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    role: Role,
) -> Result<bool> {
    if authority.key() == vault_config.admin {
        return Ok(true);
    }
    has_role(
        access_registry,
        &access_role.to_account_info(),
        &authority.to_account_info(),
        role,
    )
}
