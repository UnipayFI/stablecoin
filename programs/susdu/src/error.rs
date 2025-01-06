use anchor_lang::prelude::*;

#[error_code]
pub enum SusduError {
    /// config errors
    #[msg("Config already initialized")]
    ConfigAlreadyInitialized,
    #[msg("Config not initialized")]
    ConfigNotInitialized,
    #[msg("Config already setup susdu")]
    ConfigAlreadySetupSusdu, 
    #[msg("Config not setup susdu")]
    ConfigNotSetupSusdu,
    #[msg("Insufficient susdu")]
    InsufficientSusdu,

    // access role
    #[msg("Access role not initialized")]
    AccessRoleNotInitialized,
    #[msg("Access registry mismatch")]
    AccessRegistryMismatch,
}

