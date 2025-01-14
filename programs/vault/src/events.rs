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
pub struct BlacklistAdded {
    pub user: Pubkey,
    pub is_frozen_susdu: bool,
    pub is_frozen_usdu: bool,
}