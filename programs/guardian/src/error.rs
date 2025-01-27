use anchor_lang::prelude::*;

#[error_code]
pub enum GuardianError {
    // Mustbe Role Errors
    #[msg("Must be usdu minter")]
    MustBeUsduMinter,
    #[msg("Must be usdu redeemer")]
    MustBeUsduRedeemer,
    #[msg("Must be susdu minter")]
    MustBeSusduMinter,
    #[msg("Must be susdu redeemer")]
    MustBeSusduRedeemer,
    #[msg("Must be vault usdu minter")]
    MustBeVaultUsduMinter,
    #[msg("Must be vault usdu redeemer")]
    MustBeVaultUsduRedeemer,
    #[msg("Must be vault susdu minter")]
    MustBeVaultSusduMinter,
    #[msg("Must be vault susdu redeemer")]
    MustBeVaultSusduRedeemer,

    // Access Registry Errors
    #[msg("Access registry already initialized")]
    AccessRegistryAlreadyInitialized,
    #[msg("Access registry not initialized")]
    AccessRegistryNotInitialized,
    #[msg("Must be access registry admin")]
    MustBeAccessRegistryAdmin,

    // Access Role Errors
    #[msg("Access role already initialized")]
    AccessRoleAlreadyInitialized,
    #[msg("Access role not initialized")]
    AccessRoleNotInitialized,
    #[msg("Must be access registry")]
    MustBeAccessRegistry,
    #[msg("Invalid program id")]
    InvalidProgramId,
    #[msg("Invalid right to assign role")]
    InvalidRightToAssignRole,
    #[msg("Invalid right to revoke role")]
    InvalidRightToRevokeRole,
}
