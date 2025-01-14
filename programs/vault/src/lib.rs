#[allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;
pub mod math;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use events::*;
pub use instructions::*;
pub use math::*;
pub use state::*;
pub use utils::*;

declare_id!("5iWz5vx46Wykbu9kjArcJftsECRKPqND1WqRqyXz7TNU");

#[program]
pub mod vault {
    use super::*;

    pub fn init_vault_config(ctx: Context<InitVaultConfig>, cooldown_duration: u64) -> Result<()> {
        process_init_vault_config(ctx, cooldown_duration)
    }

    pub fn init_vault_state(ctx: Context<InitVaultState>) -> Result<()> {
        process_init_vault_state(ctx)
    }

    pub fn init_vault_state_usdu_token_account(ctx: Context<InitVaultStateUsduTokenAccount>) -> Result<()> {
        process_init_vault_state_usdu_token_account(ctx)
    }

    pub fn init_vault_state_susdu_token_account(ctx: Context<InitVaultStateSusduTokenAccount>) -> Result<()> {
        process_init_vault_state_susdu_token_account(ctx)
    }

    pub fn init_vault_state_stake_pool_usdu_token_account(ctx: Context<InitVaultStateStakePoolUsduTokenAccount>) -> Result<()> {
        process_init_vault_state_stake_pool_usdu_token_account(ctx)
    }

    pub fn init_vault_state_slio_usdu_token_account(ctx: Context<InitVaultStateSlioUsduTokenAccount>) -> Result<()> {
        process_init_vault_state_slio_usdu_token_account(ctx)
    }

    pub fn adjust_cooldown(ctx: Context<AdjustCooldown>, cooldown_duration: u64) -> Result<()> {
        process_adjust_cooldown(ctx, cooldown_duration)
    }

    pub fn deposit_collateral_mint_usdu(ctx: Context<DepositCollateralMintUsdu>, collateral_amount: u64, usdu_amount: u64) -> Result<()> {
        process_deposit_collateral_mint_usdu(ctx, collateral_amount, usdu_amount)
    }

    pub fn redeem_usdu_withdraw_collateral(ctx: Context<RedeemUsduWithdrawCollateral>, collateral_amount: u64, usdu_amount: u64) -> Result<()> {
        process_redeem_usdu_withdraw_collateral(ctx, collateral_amount, usdu_amount)
    }

    pub fn stake_usdu_mint_susdu(ctx: Context<StakeUsduMintSusdu>, usdu_amount: u64) -> Result<()> {
        process_stake_usdu_mint_susdu(ctx, usdu_amount)
    }

    pub fn unstake_susdu(ctx: Context<UnstakeSusdu>, susdu_amount: u64) -> Result<()> {
        process_unstake_susdu(ctx, susdu_amount)
    }

    pub fn withdraw_usdu(ctx: Context<WithdrawUsdu>) -> Result<()> {
        process_withdraw_usdu(ctx)
    }

    pub fn distribute_usdu_reward(ctx: Context<DistributeUsduReward>, usdu_amount: u64) -> Result<()> {
        process_distribute_usdu_reward(ctx, usdu_amount)
    }

    pub fn emergency_withdraw_vault_susdu(ctx: Context<EmergencyWithdrawVaultSusdu>, amount: u64) -> Result<()> {
        process_emergency_withdraw_vault_susdu(ctx, amount)
    }

    pub fn emergency_withdraw_vault_usdu(ctx: Context<EmergencyWithdrawVaultUsdu>, amount: u64) -> Result<()> {
        process_emergency_withdraw_vault_usdu(ctx, amount)
    }

    pub fn emergency_withdraw_vault_slio_usdu(ctx: Context<EmergencyWithdrawVaultSlioUsdu>, amount: u64) -> Result<()> {
        process_emergency_withdraw_vault_slio_usdu(ctx, amount)
    }

    pub fn emergency_withdraw_vault_stake_pool_usdu(ctx: Context<EmergencyWithdrawVaultStakePoolUsdu>, amount: u64) -> Result<()> {
        process_emergency_withdraw_vault_stake_pool_usdu(ctx, amount)
    }

    pub fn adjust_blacklist(ctx: Context<AdjustBlacklist>, user: Pubkey, is_frozen_susdu: bool, is_frozen_usdu: bool) -> Result<()> {
        process_adjust_blacklist(ctx, user, is_frozen_susdu, is_frozen_usdu)
    }

    pub fn redistribute_locked(ctx: Context<RedistributeLocked>, receiver: Pubkey) -> Result<()> {
        process_redistribute_locked(ctx, receiver)
    }
}
