use anchor_lang::prelude::*;

use crate::state::Role;

#[event]
pub struct AccessRegistryInitialized {
    pub access_registry: Pubkey,
}

#[event]
pub struct AccessRoleAssigned {
    pub role: Role,
    pub address: Pubkey,
}

#[event]
pub struct AccessRoleRevoked {
    pub role: Role,
    pub address: Pubkey,
}
