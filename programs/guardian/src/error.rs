use anchor_lang::prelude::*;

#[error_code]
pub enum GuardianError {
    // Role permission related errors
    #[msg("Unauthorized: must be guardian admin")]
    UnauthorizedGuardianAdmin,

    // USDU related role errors
    #[msg("Unauthorized: must be usdu minter")]
    UnauthorizedUsduMinter,
    #[msg("Unauthorized: must be usdu redeemer")]
    UnauthorizedUsduRedeemer,

    // SUSDU related role errors
    #[msg("Unauthorized: must be susdu minter")]
    UnauthorizedSusduMinter,
    #[msg("Unauthorized: must be susdu redeemer")]
    UnauthorizedSusduRedeemer,
    #[msg("Unauthorized: must be susdu distributor")]
    UnauthorizedSusduDistributor,

    // Vault related role errors
    #[msg("Unauthorized: must be collateral depositor")]
    UnauthorizedCollateralDepositor,
    #[msg("Unauthorized: must be collateral withdrawer")]
    UnauthorizedCollateralWithdrawer,
    #[msg("Unauthorized: must be usdu staker")]
    UnauthorizedUsduStaker,
    #[msg("Unauthorized: must be usdu unstaker")]
    UnauthorizedUsduUnstaker,
    #[msg("Unauthorized: must be vault admin")]
    UnauthorizedVaultAdmin,
    #[msg("Unauthorized: must be reward distributor")]
    UnauthorizedRewardDistributor,

    // General role errors
    #[msg("Unauthorized: must have required role")]
    UnauthorizedRole,

    // Access registry related errors
    #[msg("Access registry already initialized")]
    AccessRegistryAlreadyInitialized,
    #[msg("Access registry not initialized")]
    AccessRegistryNotInitialized,
    #[msg("Must be access registry")]
    MustBeAccessRegistry,

    // Access role related errors
    #[msg("Access role already initialized")]
    AccessRoleAlreadyInitialized,
    #[msg("Access role not initialized")]
    AccessRoleNotInitialized,
    #[msg("Role already assigned")]
    RoleAlreadyAssigned,
    #[msg("Role not assigned")]
    RoleNotAssigned,

    // Program related errors
    #[msg("Invalid program id")]
    InvalidProgramId,
    #[msg("Invalid role type")]
    InvalidRoleType,

    // Permission related errors
    #[msg("Invalid right to assign role")]
    InvalidRightToAssignRole,
    #[msg("Invalid right to revoke role")]
    InvalidRightToRevokeRole,

    // Admin transfer related errors
    #[msg("Only the current admin can propose a new admin")]
    OnlyAdminCanProposeNewAdmin,
    #[msg("Only the proposed admin can accept the transfer")]
    OnlyProposedAdminCanAccept,
    #[msg("No pending admin transfer")]
    NoPendingAdminTransfer,
}
