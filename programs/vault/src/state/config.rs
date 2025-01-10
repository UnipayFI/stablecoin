use anchor_lang::prelude::*;

use crate::error::VaultError;
use crate::math::Rounding;

#[constant]
pub const VESTING_PERIOD: u64 = 60 * 60 * 8; // 8 hours

#[constant]
pub const MIN_SHARES: u64 = 10_u64.pow(6);

#[account]
#[derive(Debug, Default)]
pub struct VaultConfig {
    pub is_initialized: bool,
    pub bump: u8,

    pub admin: Pubkey,
    pub usdu: Pubkey,
    pub susdu: Pubkey,
    pub access_registry: Pubkey,

    pub cooldown_duration: u64,
    pub total_usdu_supply: u64,
    pub vesting_amount: u64,
    pub last_distribution_timestamp: u64,
}

#[account]
#[derive(Debug, Default)]
pub struct VaultState {
    pub is_initialized: bool,
    pub bump: u8,

    pub admin: Pubkey,

    pub vault_susdu_token_account_bump: u8,
    pub vault_usdu_token_account_bump: u8,
    pub vault_stake_pool_usdu_token_account_bump: u8,
    pub vault_slio_usdu_token_account_bump: u8,

    pub vault_susdu_token_account: Pubkey,
    pub vault_usdu_token_account: Pubkey,
    pub vault_stake_pool_usdu_token_account: Pubkey,
    pub vault_slio_usdu_token_account: Pubkey,
}

impl VaultState {
    pub const SIZE: usize = 8 + std::mem::size_of::<Self>();
}

impl VaultConfig {
    pub const SIZE: usize = 8 + std::mem::size_of::<Self>();

    pub fn total_assets(&self) -> u64 {
        let unvested_amount = self.get_unvested_amount();
        let result = self.total_usdu_supply
            .checked_sub(unvested_amount)
            .expect("Math overflow");
        msg!("total asset about usdu: {}", self.total_usdu_supply);
        msg!("unvested amount: {}", unvested_amount);
        msg!("result: {}", result);
        result
    }

    pub fn check_min_shares(&self, total_shares: u64) -> Result<()> {
        if total_shares == 0 {
            return Ok(());
        }
        require!(total_shares >= MIN_SHARES, VaultError::InsufficientMinShares);
        Ok(())
    }

    pub fn get_unvested_amount(&self) -> u64 {
        let time_since_last_distribution = (Clock::get().unwrap().unix_timestamp as u128)
            .checked_sub(self.last_distribution_timestamp as u128)
            .expect("Math overflow");
        if time_since_last_distribution >= VESTING_PERIOD as u128 {
            return 0;
        } else {
            let vesting_amount = self.vesting_amount as u128;
            let vesting_period = VESTING_PERIOD as u128;
            let result = (vesting_period
                .checked_sub(time_since_last_distribution)
                .expect("Math overflow")
                .checked_mul(vesting_amount)
                .expect("Math overflow")
                .checked_div(vesting_period)
                .expect("Math overflow")) as u64;
            msg!("vesting_amount: {}", vesting_amount);
            msg!("vesting_period: {}", vesting_period);
            msg!("time_since_last_distribution: {}", time_since_last_distribution);
            msg!("result: {}", result);
            return result;
        }
    }

    /// Convert assets to shares
    /*
        /// rounding is Math.Rounding.Floor
        /// _decimalsOffset() is 0
        _convertToShares(uint256 assets, Math.Rounding rounding) internal pure returns (uint256) {
            return assets.mulDiv(totalShares() + 10 ** _decimalsOffset(), totalAssets() + 1, rounding);
        }
    */
    fn convert_to_shares(
        &self,
        assets: u64,
        total_shares: u64,
        rounding: Rounding,
    ) -> u64 {
        let numerator = (assets as u128)
            .checked_mul(total_shares as u128 + 1u128)
            .expect("Math overflow");
        let denominator = self.total_assets() as u128 + 1u128;
        let result = numerator / denominator;
        msg!("numerator: {}", numerator);
        msg!("denominator: {}", denominator);
        msg!("result: {}", result);
        match rounding {
            Rounding::Floor => result as u64,
            Rounding::Ceil => {
                let quotient = result;
                let remainder = numerator % denominator;
                if remainder > 0 {
                    (quotient + 1) as u64
                } else {
                    quotient as u64
                }
            }
        }
    }

    /// Convert shares to assets
    /*
        /// rounding is Math.Rounding.Floor
        /// _decimalsOffset() is 0
        _convertToAssets(uint256 shares, Math.Rounding rounding) internal pure returns (uint256) {
            return shares.mulDiv(totalAssets() + 1, totalShares() + 10 ** _decimalsOffset(), rounding);
        }
    */
    fn convert_to_assets(
        &self,
        shares: u64,
        total_shares: u64,
        rounding: Rounding,
    ) -> u64 {
        let numerator = (shares as u128)
            .checked_mul(self.total_assets() as u128 + 1u128)
            .expect("Math overflow");
        let denominator = total_shares as u128 + 1u128;
        let result = numerator / denominator;
        msg!("numerator: {}", numerator);
        msg!("denominator: {}", denominator);
        msg!("result: {}", result);
        match rounding {
            Rounding::Floor => result as u64,
            Rounding::Ceil => {
                let quotient = result;
                let remainder = numerator % denominator;
                if remainder > 0 {
                    (quotient + 1) as u64
                } else {
                    quotient as u64
                }
            }
        }
    }

    /// ERC4626 preview deposit
    pub(crate) fn preview_deposit(&self, assets: u64, total_shares: u64) -> u64 {
        self.convert_to_shares(assets, total_shares, Rounding::Floor)
    }

    /// ERC4626 preview withdraw
    // pub(crate) fn preview_withdraw(&self, assets: u128, total_shares: u128) -> u128 {
    //     self.convert_to_shares(assets, total_shares, true)
    // }
    
    /// ERC4626 preview mint
    // pub(crate) fn preview_mint(&self, shares: u128, total_shares: u128) -> u128 {
    //     self.convert_to_assets(shares, total_shares, false)
    // }

    /// ERC4626 preview redeem
    pub(crate) fn preview_redeem(&self, shares: u64, total_shares: u64) -> u64 {
        self.convert_to_assets(shares, total_shares, Rounding::Floor)
    }

    /// ERC4626 max_deposit
    pub(crate) fn max_deposit(&self) -> u64 {
        u64::MAX
    }
}
