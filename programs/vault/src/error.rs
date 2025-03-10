use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    // Configuration related errors
    #[msg("Config already initialized")]
    ConfigAlreadyInitialized,
    #[msg("Config not initialized")]
    ConfigNotInitialized,
    #[msg("State not initialized")]
    StateNotInitialized,
    #[msg("State already initialized")]
    StateAlreadyInitialized,
    #[msg("Initial deposit required")]
    InitialDepositRequired,
    #[msg("Initial deposit already added")]
    InitialDepositAlreadyAdded,
    #[msg("Insufficient initial deposit")]
    InsufficientInitialDeposit,

    // Permission related errors
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Unauthorized role")]
    UnauthorizedRole,
    #[msg("Invalid vault admin authority")]
    InvalidVaultAdminAuthority,
    #[msg("Vault state admin mismatch")]
    VaultStateAdminMismatch,
    #[msg("No delegate")]
    NoDelegate,
    #[msg("Delegate account mismatch")]
    DelegateAccountMismatch,
    #[msg("Only the current admin can propose a new admin")]
    OnlyAdminCanProposeNewAdmin,
    #[msg("Only the proposed admin can accept the transfer")]
    OnlyProposedAdminCanAccept,
    #[msg("No pending admin transfer")]
    NoPendingAdminTransfer,

    // Vault related errors
    #[msg("Collateral mismatch")]
    CollateralMismatch,
    #[msg("Invalid collateral token")]
    InvalidCollateralToken,
    #[msg("Insufficient collateral")]
    InsufficientCollateral,
    #[msg("Invalid post amount input")]
    InvalidPostAmountInput,
    #[msg("Max deposit exceeded")]
    MaxDepositExceeded,
    #[msg("Insufficient min shares")]
    InsufficientMinShares,
    #[msg("Invalid usdu token")]
    InvalidUsduToken,
    #[msg("Invalid susdu token")]
    InvalidSusduToken,

    // Vault account related errors
    #[msg("Invalid vault stake pool usdu token account")]
    InvalidVaultStakePoolUsduTokenAccount,
    #[msg("Invalid vault silo usdu token account")]
    InvalidVaultSiloUsduTokenAccount,
    #[msg("Invalid vault usdu token account")]
    InvalidVaultUsduTokenAccount,
    #[msg("Invalid vault susdu token account")]
    InvalidVaultSusduTokenAccount,
    #[msg("Vault usdu token account already initialized")]
    VaultUsduTokenAccountAlreadyInitialized,
    #[msg("Vault susdu token account already initialized")]
    VaultSusduTokenAccountAlreadyInitialized,
    #[msg("Vault stake pool usdu token account already initialized")]
    VaultStakePoolUsduTokenAccountAlreadyInitialized,
    #[msg("Vault silo usdu token account already initialized")]
    VaultSiloUsduTokenAccountAlreadyInitialized,
    #[msg("Invalid receiver usdu token account")]
    InvalidReceiverUsduTokenAccount,

    // Cooldown related errors
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
    #[msg("Cooldown duration too short")]
    CooldownDurationTooShort,
    #[msg("Cooldown duration too long")]
    CooldownDurationTooLong,

    // USDU related errors
    #[msg("Amount must be greater than zero")]
    AmountMustBeGreaterThanZero,
    #[msg("Insufficient usdu")]
    InsufficientUsdu,
    #[msg("Insufficient usdu supply")]
    InsufficientUsduSupply,
    #[msg("Insufficient usdu balance")]
    InsufficientUsduBalance,
    #[msg("Insufficient unvested usdu")]
    InsufficientUnvestedUsdu,
    #[msg("Invalid stake usdu amount")]
    InvalidStakeUsduAmount,
    #[msg("Invalid preview redeem usdu amount")]
    InvalidPreviewRedeemUsduAmount,
    #[msg("Insufficient vault usdu")]
    InsufficientVaultUsdu,
    #[msg("Insufficient stake pool usdu")]
    InsufficientStakePoolUsdu,
    #[msg("Insufficient silo usdu")]
    InsufficientSiloUsdu,
    #[msg("Insufficient usdu in silo for cooldown withdrawals")]
    InsufficientUsduInSilo,
    #[msg("Still vesting")]
    StillVesting,

    // SUSDU related errors
    #[msg("Insufficient susdu")]
    InsufficientSusdu,
    #[msg("Insufficient vault susdu")]
    InsufficientVaultSusdu,
    #[msg("Susdu total supply too low")]
    SusduTotalSupplyTooLow,
    #[msg("Invalid unstake susdu amount")]
    InvalidUnstakeSusduAmount,
    #[msg("Invalid preview deposit susdu amount")]
    InvalidPreviewDepositSusduAmount,
    #[msg("Invalid locked susdu token account owner")]
    InvalidLockedSusduTokenAccountOwner,
    #[msg("Invalid locked susdu token account amount")]
    InvalidLockedSusduTokenAccountAmount,

    // Access control related errors
    #[msg("Access role not initialized")]
    AccessRoleNotInitialized,
    #[msg("Access registry mismatch")]
    AccessRegistryMismatch,

    // Blacklist related errors
    #[msg("Not blacklist account")]
    NotBlacklistAccount,

    // Math calculation related errors
    #[msg("Math overflow")]
    MathOverflow,
}
