use anchor_lang::prelude::*;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum Role {

    // USDU roles
    UsduMinter,
    UsduRedeemer,

    // SUSDU roles
    SusduMinter,
    SusduRedeemer,

    // Vault roles
    VaultUsduMinter,
    VaultUsduRedeemer,
    VaultSusduMinter,
    VaultSusduRedeemer,
    VaultManager,
    DistributeRewarder,
}

impl Role {
    pub fn to_seed(&self) -> [u8; 32] {
        let mut seed = [0u8; 32];
        seed[..self.to_string().len()].copy_from_slice(self.to_string().as_bytes());
        seed
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let role_str = match self {
            Role::UsduMinter => "usdu_minter",
            Role::UsduRedeemer => "usdu_redeemer",

            Role::SusduMinter => "susdu_minter",
            Role::SusduRedeemer => "susdu_redeemer",

            Role::VaultUsduMinter => "vault_usdu_minter",
            Role::VaultUsduRedeemer => "vault_usdu_redeemer",
            Role::VaultSusduMinter => "vault_susdu_minter",
            Role::VaultSusduRedeemer => "vault_susdu_redeemer",
            Role::VaultManager => "vault_manager",
            Role::DistributeRewarder => "distribute_rewarder",
        };
        write!(f, "{}", role_str)
    }
}

#[account]
#[derive(Debug, InitSpace)]
pub struct AccessRegistry {
    pub admin: Pubkey,
    pub bump: u8,
    pub is_initialized: bool,
}

impl AccessRegistry {
    pub const SIZE: usize = 8 + Self::INIT_SPACE;
}

#[account]
pub struct AccessRole {
    pub owner: Pubkey,
    pub role: Role,
    pub bump: u8,
    pub is_initialized: bool,
    pub access_registry: Pubkey,
}

impl AccessRole {
    pub const SIZE: usize = 8 + std::mem::size_of::<AccessRole>();
}
