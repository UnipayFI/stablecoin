use anchor_lang::prelude::*;

#[error_code]
pub enum SusduError {
    // Config related errors
    #[msg("Config already initialized")]
    ConfigAlreadyInitialized,
    #[msg("Config not initialized")]
    ConfigNotInitialized,
    #[msg("Config already setup susdu")]
    ConfigAlreadySetupSusdu,
    #[msg("Config not setup susdu")]
    ConfigNotSetupSusdu,

    // Admin transfer related errors
    #[msg("Proposed admin already set")]
    ProposedAdminAlreadySet,
    #[msg("Proposed admin is current admin")]
    ProposedAdminIsCurrentAdmin,

    // Permission related errors
    #[msg("Unauthorized role")]
    UnauthorizedRole,
    #[msg("Invalid admin authority")]
    InvalidAdminAuthority,

    // Token related errors
    #[msg("Insufficient susdu")]
    InsufficientSusdu,
    #[msg("Amount must be greater than zero")]
    AmountMustBeGreaterThanZero,
    #[msg("Invalid susdu amount")]
    InvalidSusduAmount,
    #[msg("Invalid susdu token account")]
    InvalidSusduTokenAccount,
    #[msg("Invalid receiver susdu token account")]
    InvalidReceiverSusduTokenAccount,
    #[msg("Invalid susdu token")]
    InvalidSusduToken,

    // Access control related errors
    #[msg("Access role not initialized")]
    AccessRoleNotInitialized,
    #[msg("Access registry mismatch")]
    AccessRegistryMismatch,

    // Math calculation related errors
    #[msg("Math overflow")]
    MathOverflow,

    // Admin transfer related errors
    #[msg("Only the current admin can propose a new admin")]
    OnlyAdminCanProposeNewAdmin,
    #[msg("Only the proposed admin can accept the transfer")]
    OnlyProposedAdminCanAccept,
    #[msg("No pending admin transfer")]
    NoPendingAdminTransfer,
}
