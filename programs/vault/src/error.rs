use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("Config already initialized")]
    ConfigAlreadyInitialized,
    #[msg("Config not initialized")]
    ConfigNotInitialized,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
    #[msg("No delegate")]
    NoDelegate,
    #[msg("Delegate account mismatch")]
    DelegateAccountMismatch,
    #[msg("Invalid vault admin authority")]
    InvalidVaultAdminAuthority,

    // vault
    #[msg("Collateral mismatch")]
    CollateralMismatch,
    #[msg("Max deposit exceeded")]
    MaxDepositExceeded,
    #[msg("Invalid vault stake pool usdu token account")]
    InvalidVaultStakePoolUsduTokenAccount,
    #[msg("Invalid vault slio usdu token account")]
    InvalidVaultSlioUsduTokenAccount,
    #[msg("Invalid cooldown owner")]
    InvalidCooldownOwner,
    #[msg("Invalid cooldown underlying token account")]
    InvalidCooldownUnderlyingTokenAccount,
    #[msg("Invalid cooldown underlying token mint")]
    InvalidCooldownUnderlyingTokenMint,
    #[msg("Cooldown not initialized")]
    CooldownNotInitialized,
    #[msg("Cooldown active")]
    CooldownActive,

    // access role
    #[msg("Access role not initialized")]
    AccessRoleNotInitialized,
    #[msg("Access registry mismatch")]
    AccessRegistryMismatch,
    #[msg("Invalid receiver usdu token account")]
    InvalidReceiverUsduTokenAccount,

    // susdu
    #[msg("Insufficient susdu")]
    InsufficientSusdu,

    // usdu
    #[msg("Insufficient usdu can not be zero")]
    InsufficientUsduCanNotBeZero,
    #[msg("Amount must be greater than zero")]
    AmountMustBeGreaterThanZero,
    #[msg("Insufficient usdu")]
    InsufficientUsdu,

    // emergency
    #[msg("Insufficient silo usdu")]
    InsufficientSlioUsdu,
    #[msg("Insufficient stake pool usdu")]
    InsufficientStakePoolUsdu,
    #[msg("Insufficient vault usdu")]
    InsufficientVaultUsdu,
    #[msg("Insufficient vault susdu")]
    InsufficientVaultSusdu,

    #[msg("State not initialized")]
    StateNotInitialized,
    #[msg("State already initialized")]
    StateAlreadyInitialized,
    #[msg("Vault state admin mismatch")]
    VaultStateAdminMismatch,

    #[msg("Vault usdu token account already initialized")]
    VaultUsduTokenAccountAlreadyInitialized,
    #[msg("Vault susdu token account already initialized")]
    VaultSusduTokenAccountAlreadyInitialized,
    #[msg("Vault stake pool usdu token account already initialized")]
    VaultStakePoolUsduTokenAccountAlreadyInitialized,
    #[msg("Vault slio usdu token account already initialized")]
    VaultSlioUsduTokenAccountAlreadyInitialized,

    #[msg("Insufficient usdu supply")]
    InsufficientUsduSupply,
    #[msg("Invalid preview deposit susdu amount")]
    InvalidPreviewDepositSusduAmount,
    #[msg("Insufficient usdu balance")]
    InsufficientUsduBalance,
    #[msg("Insufficient unvested usdu")]
    InsufficientUnvestedUsdu,
    #[msg("Insufficient min shares")]
    InsufficientMinShares,
    #[msg("Still vesting")]
    StillVesting,
}