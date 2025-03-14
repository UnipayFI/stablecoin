#[allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod math;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use events::*;
pub use instructions::*;
pub use math::*;
pub use state::*;
pub use utils::*;

declare_id!("GHQwoTgsKdsKMjn3ysdtWBNRbvzoiTGzBAuJn4idZB4");

#[program]
pub mod vault {
    use super::*;

    pub fn init_vault_config(ctx: Context<InitVaultConfig>, cooldown_duration: u64) -> Result<()> {
        process_init_vault_config(ctx, cooldown_duration)
    }

    pub fn init_vault_state(ctx: Context<InitVaultState>) -> Result<()> {
        process_init_vault_state(ctx)
    }

    pub fn init_vault_state_usdu_token_account(
        ctx: Context<InitVaultStateUsduTokenAccount>,
    ) -> Result<()> {
        process_init_vault_state_usdu_token_account(ctx)
    }

    pub fn init_vault_state_susdu_token_account(
        ctx: Context<InitVaultStateSusduTokenAccount>,
    ) -> Result<()> {
        process_init_vault_state_susdu_token_account(ctx)
    }

    pub fn init_vault_state_stake_pool_usdu_token_account(
        ctx: Context<InitVaultStateStakePoolUsduTokenAccount>,
    ) -> Result<()> {
        process_init_vault_state_stake_pool_usdu_token_account(ctx)
    }

    pub fn init_vault_state_slio_usdu_token_account(
        ctx: Context<InitVaultStateSlioUsduTokenAccount>,
    ) -> Result<()> {
        process_init_vault_state_slio_usdu_token_account(ctx)
    }

    pub fn adjust_cooldown(ctx: Context<AdjustCooldown>, cooldown_duration: u64) -> Result<()> {
        process_adjust_cooldown(ctx, cooldown_duration)
    }

    pub fn deposit_collateral_mint_usdu(
        ctx: Context<DepositCollateralMintUsdu>,
        collateral_amount: u64,
        usdu_amount: u64,
    ) -> Result<()> {
        process_deposit_collateral_mint_usdu(ctx, collateral_amount, usdu_amount)
    }

    pub fn redeem_usdu_withdraw_collateral(
        ctx: Context<RedeemUsduWithdrawCollateral>,
        collateral_amount: u64,
        usdu_amount: u64,
    ) -> Result<()> {
        process_redeem_usdu_withdraw_collateral(ctx, collateral_amount, usdu_amount)
    }

    pub fn stake_usdu_mint_susdu(ctx: Context<StakeUsduMintSusdu>, usdu_amount: u64) -> Result<()> {
        process_stake_usdu_mint_susdu(ctx, usdu_amount)
    }

    pub fn unstake_susdu<'info>(
        ctx: Context<'_, '_, '_, 'info, UnstakeSusdu<'info>>,
        susdu_amount: u64,
    ) -> Result<()> {
        process_unstake_susdu(ctx, susdu_amount)
    }

    pub fn withdraw_usdu(ctx: Context<WithdrawUsdu>) -> Result<()> {
        process_withdraw_usdu(ctx)
    }

    pub fn distribute_usdu_reward(
        ctx: Context<DistributeUsduReward>,
        usdu_amount: u64,
    ) -> Result<()> {
        process_distribute_usdu_reward(ctx, usdu_amount)
    }

    pub fn emergency_withdraw_vault_susdu<'info>(
        ctx: Context<'_, '_, '_, 'info, EmergencyWithdrawVaultSusdu<'info>>,
        amount: u64,
    ) -> Result<()> {
        process_emergency_withdraw_vault_susdu(ctx, amount)
    }

    pub fn emergency_withdraw_vault_usdu(
        ctx: Context<EmergencyWithdrawVaultUsdu>,
        amount: u64,
    ) -> Result<()> {
        process_emergency_withdraw_vault_usdu(ctx, amount)
    }

    pub fn emergency_withdraw_vault_slio_usdu(
        ctx: Context<EmergencyWithdrawVaultSlioUsdu>,
        amount: u64,
    ) -> Result<()> {
        process_emergency_withdraw_vault_slio_usdu(ctx, amount)
    }

    pub fn emergency_withdraw_vault_stake_pool_usdu(
        ctx: Context<EmergencyWithdrawVaultStakePoolUsdu>,
        amount: u64,
    ) -> Result<()> {
        process_emergency_withdraw_vault_stake_pool_usdu(ctx, amount)
    }

    pub fn redistribute_locked(ctx: Context<RedistributeLocked>) -> Result<()> {
        process_redistribute_locked(ctx)
    }

    pub fn propose_new_admin(ctx: Context<ProposeNewAdmin>) -> Result<()> {
        process_propose_new_admin(ctx)
    }

    pub fn accept_admin_transfer(ctx: Context<AcceptAdminTransfer>) -> Result<()> {
        process_accept_admin_transfer(ctx)
    }
}
