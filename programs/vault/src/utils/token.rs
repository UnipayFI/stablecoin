use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    token_2022::{
        spl_token_2022::{
            self,
            extension::{
                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
            },
        },
    },
    token_interface::Mint,
};

#[cfg(feature = "devnet-mint")]
const MINT_WHITELIST: [&'static str; 1] = [
    "3FoZrkGxVDs9qVLtyzYm9rdFtpQL481ZvomnWf6f2qv9",
];

#[cfg(feature = "mainnet-mint")]
const MINT_WHITELIST: [&'static str; 1] = [
    "So11111111111111111111111111111111111111112",
];

pub fn is_supported_mint(mint_account: &InterfaceAccount<Mint>) -> Result<bool> {
    #[cfg(feature = "whitelist-mint")]
    {
        use std::collections::HashSet;

        let mint_whitelist: HashSet<&str> = MINT_WHITELIST.into_iter().collect();
        if mint_whitelist.contains(mint_account.key().to_string().as_str()) {
            return Ok(true);
        }
        return Ok(false);
    }
    #[cfg(not(feature = "whitelist-mint"))]
    {
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
