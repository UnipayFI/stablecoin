use anchor_lang::prelude::*;

use crate::{Role, AccessRegistry, AccessRole};
use crate::constants::{ACCESS_ROLE_SEED};
use crate::error::GuardianError;

pub fn has_role<'info>(
    access_registry: &Account<AccessRegistry>,
    access_role: &AccountInfo<'info>,
    authority: &AccountInfo<'info>,
    role: Role,
) -> Result<bool> {

    if authority.key() == access_registry.admin {
        return Ok(true);
    }

    if access_role.owner != &crate::ID {
        return Ok(false);
    }

    let matched_role = match AccessRole::try_deserialize(&mut &access_role.data.borrow()[..]) {
        Ok(matched_role) => matched_role,
        Err(_) => return Ok(false),
    };

    require!(access_registry.is_initialized, GuardianError::AccessRegistryNotInitialized);
    require!(matched_role.is_initialized, GuardianError::AccessRoleNotInitialized);
    require!(matched_role.access_registry == access_registry.key(), GuardianError::MustBeAccessRegistry);
    require!(access_registry.to_account_info().owner == &crate::ID, GuardianError::InvalidProgramId);

    let (role_address, _) = Pubkey::find_program_address(
        &[
            ACCESS_ROLE_SEED,
            access_registry.key().as_ref(),
            authority.key().as_ref(),
            role.to_seed().as_slice(),
        ],
        &crate::ID,
    );
    if access_role.key() == role_address || (authority.key() == matched_role.owner && matched_role.role == role) {
        return Ok(true);
    }

    Ok(false)
}

