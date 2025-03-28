use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, rent::Rent, system_instruction::transfer};
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{BaseStateWithExtensions, Extension, StateWithExtensions},
        state::Mint as StateMint,
    },
    token_interface::{
        spl_token_metadata_interface::state::TokenMetadata, token_metadata_initialize, Mint,
        Token2022, TokenMetadataInitialize,
    },
};
use spl_type_length_value::variable_len_pack::VariableLenPack;

use crate::constants::{USDU_CONFIG_SEED, USDU_SEED};
use crate::error::UsduError;
use crate::events::UsduTokenCreated;
use crate::state::UsduConfig;

fn get_mint_extensible_with_len_extension<T: Extension + VariableLenPack>(
    mint_account: &mut AccountInfo,
) -> Result<T> {
    let mint_data = mint_account.data.borrow();
    let mint_with_extension = StateWithExtensions::<StateMint>::unpack(&mint_data)?;
    let extension_data = mint_with_extension.get_variable_len_extension::<T>()?;
    Ok(extension_data)
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateUsdu<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [USDU_CONFIG_SEED],
        bump = usdu_config.bump,
        has_one = admin,
    )]
    pub usdu_config: Box<Account<'info, UsduConfig>>,
    #[account(
        init_if_needed,
        payer = admin,
        seeds = [USDU_SEED],
        bump,
        mint::authority = usdu_token,
        mint::decimals = decimals,
        mint::token_program = token_program,
        extensions::metadata_pointer::authority = usdu_token,
        extensions::metadata_pointer::metadata_address = usdu_token,
    )]
    pub usdu_token: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_create_usdu(ctx: Context<CreateUsdu>, decimals: u8) -> Result<()> {
    require!(
        !ctx.accounts.usdu_config.is_usdu_token_initialized,
        UsduError::ConfigAlreadySetupUsdu
    );

    ctx.accounts.usdu_config.usdu_token = ctx.accounts.usdu_token.key();
    ctx.accounts.usdu_config.usdu_token_bump = ctx.bumps.usdu_token;
    ctx.accounts.usdu_config.is_usdu_token_initialized = true;
    ctx.accounts.usdu_config.total_supply = 0;

    let name = "USDU - Program Controlled Token".to_string();
    let symbol = "USDU".to_string();
    let uri = "https://bafybeib5rbwqc5hj52hhc6k6g4c5qfhlq2jkkeujypc3okvm7dqoypgcku.ipfs.w3s.link/usdu.png".to_string();

    // create metadata account
    let usdu_token_bump = &[ctx.bumps.usdu_token];
    let seeds = &[&[USDU_SEED, usdu_token_bump][..]];
    token_metadata_initialize(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataInitialize {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.usdu_token.to_account_info(),
                metadata: ctx.accounts.usdu_token.to_account_info(),
                mint_authority: ctx.accounts.usdu_token.to_account_info(),
                update_authority: ctx.accounts.usdu_token.to_account_info(),
            },
            seeds,
        ),
        name.clone(),
        symbol.clone(),
        uri.clone(),
    )?;

    ctx.accounts.usdu_token.reload()?;
    let usdu_token_account = &mut ctx.accounts.usdu_token.to_account_info();
    let metadata = get_mint_extensible_with_len_extension::<TokenMetadata>(usdu_token_account)?;
    assert_eq!(metadata.mint, ctx.accounts.usdu_token.key());
    assert_eq!(metadata.name, name);
    assert_eq!(metadata.symbol, symbol);
    assert_eq!(metadata.uri, uri);

    // transfer rent to usdu_token
    let extra_lamports =
        Rent::get()?.minimum_balance(usdu_token_account.data_len()) - usdu_token_account.lamports();
    if extra_lamports > 0 {
        invoke(
            &transfer(
                &ctx.accounts.admin.key(),
                &ctx.accounts.usdu_token.key(),
                extra_lamports,
            ),
            &[
                ctx.accounts.admin.to_account_info(),
                ctx.accounts.usdu_token.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    emit!(UsduTokenCreated {
        usdu_token: ctx.accounts.usdu_token.key(),
        decimals,
    });
    Ok(())
}
