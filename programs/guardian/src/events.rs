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

#[event]
pub struct AdminTransferProposed {
    pub access_registry: Pubkey,
    pub current_admin: Pubkey,
    pub proposed_admin: Pubkey,
}

#[event]
pub struct AdminTransferCompleted {
    pub access_registry: Pubkey,
    pub previous_admin: Pubkey,
    pub new_admin: Pubkey,
}
