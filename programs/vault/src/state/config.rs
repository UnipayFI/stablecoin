use anchor_lang::prelude::*;

use crate::error::VaultError;
use crate::math::Rounding;

#[constant]
pub const VESTING_PERIOD: u64 = 60 * 60 * 8; // 8 hours

#[constant]
pub const MIN_SHARES: u64 = 10_u64.pow(6);

#[constant]
pub const INITIAL_DEPOSIT_AMOUNT: u64 = 1000 * 10_u64.pow(6);

#[account]
#[derive(Debug, Default)]
pub struct VaultConfig {
    pub is_initialized: bool,
    pub bump: u8,

    pub admin: Pubkey,
    pub pending_admin: Pubkey,
    pub usdu: Pubkey,
    pub susdu: Pubkey,
    pub access_registry: Pubkey,

    pub cooldown_duration: u64,
    pub total_staked_usdu_supply: u64,
    pub total_cooldown_usdu_amount: u64,
    pub vesting_amount: u64,
    pub last_distribution_timestamp: u64,
    pub has_initial_deposit: bool,
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
    pub vault_silo_usdu_token_account_bump: u8,

    pub vault_susdu_token_account: Pubkey,
    pub vault_usdu_token_account: Pubkey,
    pub vault_stake_pool_usdu_token_account: Pubkey,
    pub vault_silo_usdu_token_account: Pubkey,
}

impl VaultState {
    pub const SIZE: usize = 8 + std::mem::size_of::<Self>();
}

impl VaultConfig {
    pub const SIZE: usize = 8 + std::mem::size_of::<Self>();

    pub fn total_assets(&self) -> u64 {
        let unvested_amount = self.get_unvested_amount();
        let result = self
            .total_staked_usdu_supply
            .checked_sub(unvested_amount)
            .expect("Math overflow");
        result
    }

    pub fn check_initial_deposit(&self, deposit_amount: u64) -> Result<()> {
        if !self.has_initial_deposit {
            require!(
                deposit_amount >= INITIAL_DEPOSIT_AMOUNT,
                VaultError::InsufficientInitialDeposit
            );
        }
        Ok(())
    }

    pub fn check_min_shares(&self, total_shares: u64) -> Result<()> {
        if !self.has_initial_deposit {
            return Ok(());
        }
        require!(
            total_shares == 0 || total_shares >= MIN_SHARES,
            VaultError::InsufficientMinShares
        );
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
    // Note: We use Rounding::Floor to ensure the vault never mints more SUSDU than it should,
    // preventing inflation. This is consistent with Ethena's implementation.
    // There will be a small precision loss (dust) due to rounding, which is an accepted trade-off
    // for increased security.
    fn convert_to_shares(&self, assets: u64, total_shares: u64, rounding: Rounding) -> u64 {
        let numerator = (assets as u128)
            .checked_mul(total_shares as u128 + 1u128)
            .expect("Math overflow");
        let denominator = self.total_assets() as u128 + 1u128;
        let result = numerator / denominator;
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
    // Note: We use Rounding::Floor to ensure consistent behavior with convert_to_shares.
    // This is consistent with Ethena's implementation.
    // There will be a small precision loss (dust) due to rounding, which is an accepted trade-off
    // for increased security.
    fn convert_to_assets(&self, shares: u64, total_shares: u64, rounding: Rounding) -> u64 {
        let numerator = (shares as u128)
            .checked_mul(self.total_assets() as u128 + 1u128)
            .expect("Math overflow");
        let denominator = total_shares as u128 + 1u128;
        let result = numerator / denominator;
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
    /// Uses Rounding::Floor to ensure the vault never mints more SUSDU than it should
    pub(crate) fn preview_deposit(&self, assets: u64, total_shares: u64) -> u64 {
        self.convert_to_shares(assets, total_shares, Rounding::Floor)
    }

    /// ERC4626 preview redeem
    /// Uses Rounding::Floor to ensure consistent behavior with preview_deposit
    pub(crate) fn preview_redeem(&self, shares: u64, total_shares: u64) -> u64 {
        self.convert_to_assets(shares, total_shares, Rounding::Floor)
    }

    /// ERC4626 max_deposit
    pub(crate) fn max_deposit(&self) -> u64 {
        u64::MAX
    }
}
