use anchor_lang::prelude::*;

#[event]
pub struct CooldownAdjusted {
    pub vault_config: Pubkey,
    pub cooldown_duration: u64,
}

#[event]
pub struct RedeemUsduWithdrawCollateralEvent {
    pub benefactor: Pubkey,
    pub beneficiary: Pubkey,
    pub fund: Pubkey,
    pub collateral_amount: u64,
    pub usdu_amount: u64,
}

#[event]
pub struct DepositCollateralMintUsduEvent {
    pub benefactor: Pubkey,
    pub beneficiary: Pubkey,
    pub fund: Pubkey,
    pub collateral_amount: u64,
    pub usdu_amount: u64,
}

#[event]
pub struct AdminTransferProposed {
    pub vault_config: Pubkey,
    pub current_admin: Pubkey,
    pub proposed_admin: Pubkey,
}

#[event]
pub struct AdminTransferCompleted {
    pub vault_config: Pubkey,
    pub previous_admin: Pubkey,
    pub new_admin: Pubkey,
}

#[event]
pub struct VaultConfigInitialized {
    pub vault_config: Pubkey,
    pub admin: Pubkey,
    pub usdu_token: Pubkey,
    pub susdu_token: Pubkey,
    pub access_registry: Pubkey,
    pub cooldown_duration: u64,
}

#[event]
pub struct VaultStateInitialized {
    pub vault_state: Pubkey,
    pub admin: Pubkey,
}

#[event]
pub struct VaultTokenAccountInitialized {
    pub vault_state: Pubkey,
    pub token_account: Pubkey,
}

#[event]
pub struct UsduRewardDistributed {
    pub vault_config: Pubkey,
    pub distributor: Pubkey,
    pub amount: u64,
    pub total_staked_usdu_supply: u64,
    pub timestamp: u64,
}

#[event]
pub struct LockedSusduRedistributed {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub is_burned: bool,
}

#[event]
pub struct UsduWithdrawn {
    pub vault_config: Pubkey,
    pub caller: Pubkey,
    pub receiver: Pubkey,
    pub usdu_amount: u64,
    pub timestamp: u64,
}

#[event]
pub struct RedistributedSusdu {
    pub vault_config: Pubkey,
    pub authority: Pubkey,
    pub amount: u64,
    pub receiver: Pubkey,
    pub timestamp: u64,
}

#[event]
pub struct EmergencyWithdrawal {
    pub vault_config: Pubkey,
    pub admin: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub destination: Pubkey,
    pub timestamp: u64,
}
