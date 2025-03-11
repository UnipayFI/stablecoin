use anchor_lang::prelude::*;

#[error_code]
pub enum BlacklistHookError {
    // Initialize related errors
    #[msg("Config already initialized")]
    ConfigAlreadyInitialized,
    #[msg("Config not initialized")]
    ConfigNotInitialized,

    // Mint related errors
    #[msg("Invalid mint")]
    InvalidMint,

    // Transfer hook related errors
    #[msg("Is not transferring")]
    IsNotTransferring,

    // Admin transfer related errors
    #[msg("Only the current admin can propose a new admin")]
    OnlyAdminCanProposeNewAdmin,
    #[msg("Only the proposed admin can accept the transfer")]
    OnlyProposedAdminCanAccept,
    #[msg("No pending admin transfer")]
    NoPendingAdminTransfer,

    // Blacklist related errors
    #[msg("Only the admin can modify the blacklist")]
    OnlyAdminCanModifyBlacklist,
    #[msg("User is already in blacklist")]
    UserAlreadyInBlacklist,
    #[msg("User is not in blacklist")]
    UserNotInBlacklist,
    #[msg("Blacklist entry already exists")]
    BlacklistEntryAlreadyExists,
    #[msg("Source address blacklisted")]
    SourceAddressBlacklisted,
    #[msg("Destination address blacklisted")]
    DestinationAddressBlacklisted,
}
