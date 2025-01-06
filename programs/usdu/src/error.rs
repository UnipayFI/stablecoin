use anchor_lang::prelude::*;

#[error_code]
pub enum UsduError {
    /// config errors
    #[msg("Config already initialized")]
    ConfigAlreadyInitialized,
    #[msg("Config not initialized")]
    ConfigNotInitialized,
    #[msg("Config already setup usdu")]
    ConfigAlreadySetupUSDU, 
    #[msg("Config not setup usdu")]
    ConfigNotSetupUSDU,
    #[msg("Insufficient usdu")]
    InsufficientUsdu,

    // access role
    #[msg("Access role not initialized")]
    AccessRoleNotInitialized,
    #[msg("Access registry mismatch")]
    AccessRegistryMismatch,
}

