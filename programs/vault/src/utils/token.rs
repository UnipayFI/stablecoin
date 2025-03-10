use crate::error::VaultError;
use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_2022::spl_token_2022::{
        self,
        extension::{
            transfer_fee::{TransferFeeConfig, MAX_FEE_BASIS_POINTS},
            BaseStateWithExtensions, StateWithExtensions,
        },
    },
    token_interface::Mint,
};

#[cfg(feature = "devnet")]
const MINT_WHITELIST: [&'static str; 1] = ["3FoZrkGxVDs9qVLtyzYm9rdFtpQL481ZvomnWf6f2qv9"];

#[cfg(feature = "mainnet")]
const MINT_WHITELIST: [&'static str; 1] = ["So11111111111111111111111111111111111111112"];

#[cfg(feature = "testnet")]
const MINT_WHITELIST: [&'static str; 1] = ["BTcP1cFC2Vy254HzFP8yAeKK2kz4fCXXGJYiMwttmNaC"];

pub fn get_transfer_inverse_fee(mint_info: &AccountInfo, post_fee_amount: u64) -> Result<u64> {
    if *mint_info.owner == Token::id() {
        return Ok(0);
    }
    if post_fee_amount == 0 {
        return err!(VaultError::InvalidPostAmountInput);
    }
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;

    let fee = if let Ok(transfer_fee_config) = mint.get_extension::<TransferFeeConfig>() {
        let epoch = Clock::get()?.epoch;

        let transfer_fee = transfer_fee_config.get_epoch_fee(epoch);
        if u16::from(transfer_fee.transfer_fee_basis_points) == MAX_FEE_BASIS_POINTS {
            u64::from(transfer_fee.maximum_fee)
        } else {
            transfer_fee_config
                .calculate_inverse_epoch_fee(epoch, post_fee_amount)
                .unwrap()
        }
    } else {
        0
    };
    Ok(fee)
}

pub fn is_supported_mint(mint_account: &InterfaceAccount<Mint>) -> Result<bool> {
    #[cfg(feature = "whitelist")]
    {
        use std::collections::HashSet;

        let mint_whitelist: HashSet<&str> = MINT_WHITELIST.into_iter().collect();
        if mint_whitelist.contains(mint_account.key().to_string().as_str()) {
            return Ok(true);
        }
        return Ok(false);
    }
    #[cfg(not(feature = "whitelist"))]
    {
        use anchor_spl::token_2022::spl_token_2022::extension::ExtensionType;

        let mint_info = mint_account.to_account_info();
        if *mint_info.owner == Token::id() {
            return Ok(true);
        }
        let mint_data = mint_info.try_borrow_data()?;
        let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
        let extensions = mint.get_extension_types()?;
        for e in extensions {
            if e != ExtensionType::TransferFeeConfig
                && e != ExtensionType::MetadataPointer
                && e != ExtensionType::TokenMetadata
            {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
