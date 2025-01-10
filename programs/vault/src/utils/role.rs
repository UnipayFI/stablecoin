use anchor_lang::prelude::*;

use crate::state::VaultConfig;

use guardian::utils::has_role;
use guardian::{Role, AccessRegistry, AccessRole};

pub fn has_role_or_admin<'info>(
    vault_config: &Account<VaultConfig>,
    access_registry: &Account<AccessRegistry>,
    access_role: &Account<AccessRole>,
    authority: &AccountInfo<'info>,
    role: Role,
) -> Result<bool> {
    if authority.key() == vault_config.admin {
        return Ok(true);
    }
    has_role(access_registry, access_role, authority, role)
}