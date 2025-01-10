use anchor_lang::prelude::*;
use crate::{Role, AccessRegistry, AccessRole};
use crate::error::GuardianError;

pub fn has_role<'info>(
    access_registry: &Account<AccessRegistry>,
    access_role: &Account<AccessRole>,
    authority: &AccountInfo<'info>,
    role: Role,
) -> Result<bool> {
    require!(access_registry.is_initialized, GuardianError::AccessRegistryNotInitialized);
    require!(access_role.is_initialized, GuardianError::AccessRoleNotInitialized);
    require!(access_role.access_registry == access_registry.key(), GuardianError::MustBeAccessRegistry);
    require!(access_registry.to_account_info().owner == &crate::ID, GuardianError::InvalidProgramId);
    require!(access_role.to_account_info().owner == &crate::ID, GuardianError::InvalidProgramId);

    if authority.key() == access_role.owner && access_role.role == role {
        return Ok(true);
    }

    Ok(false)
}

