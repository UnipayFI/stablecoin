use anchor_lang::prelude::*;

#[error_code]
pub enum UsduError {
    // Config related errors
    #[msg("Config already initialized")]
    ConfigAlreadyInitialized,
    #[msg("Config not initialized")]
    ConfigNotInitialized,
    #[msg("Config already setup usdu")]
    ConfigAlreadySetupUsdu,
    #[msg("Config not setup usdu")]
    ConfigNotSetupUsdu,

    // Permission related errors
    #[msg("Unauthorized role")]
    UnauthorizedRole,
    #[msg("Invalid admin authority")]
    InvalidAdminAuthority,

    // Token related errors
    #[msg("Insufficient usdu")]
    InsufficientUsdu,
    #[msg("Amount must be greater than zero")]
    AmountMustBeGreaterThanZero,
    #[msg("Invalid usdu amount")]
    InvalidUsduAmount,
    #[msg("Invalid usdu token account")]
    InvalidUsduTokenAccount,
    #[msg("Invalid receiver usdu token account")]
    InvalidReceiverUsduTokenAccount,
    #[msg("Invalid usdu token")]
    InvalidUsduToken,

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
