use anchor_lang::prelude::*;
use anchor_lang::solana_program::{rent::Rent, system_instruction::transfer, program::invoke};
use anchor_spl::{
    token_interface::{
        Mint, Token2022, spl_token_metadata_interface::state::TokenMetadata,
        token_metadata_initialize, TokenMetadataInitialize,
        spl_pod::optional_keys::OptionalNonZeroPubkey,
    },
    token_2022::spl_token_2022::{
        extension::{
            BaseStateWithExtensions,
            Extension,
            StateWithExtensions,
            permanent_delegate::PermanentDelegate
        },
        state::Mint as StateMint,
    },
};
use spl_type_length_value::variable_len_pack::VariableLenPack;

use crate::constants::{SUSDU_CONFIG_SEED, SUSDU_SEED};
use crate::state::SusduConfig;
use crate::error::SusduError;
use crate::events::SusduTokenCreated;

fn get_mint_with_len_extension<'a, T: Extension + VariableLenPack>(
    mint_account: &'a mut AccountInfo,
) -> Result<T> {
    let mint_data = mint_account.data.borrow();
    let mint_with_extension = StateWithExtensions::<StateMint>::unpack(&mint_data)?;
    let extension_data = mint_with_extension.get_variable_len_extension::<T>()?;
    Ok(extension_data)
}

fn get_mint_with_permanent_delegate(
    mint_account: &mut AccountInfo,
) -> Result<PermanentDelegate> {
    let mint_data = mint_account.data.borrow();
    let mint_with_extension = StateWithExtensions::<StateMint>::unpack(&mint_data)?;
    let extension_data = mint_with_extension.get_extension::<PermanentDelegate>()?;
    Ok(*extension_data)
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateSusdu<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
        has_one = admin,
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,

    #[account(
        init_if_needed,
        payer = admin,
        seeds = [SUSDU_SEED],
        bump,
        mint::freeze_authority = susdu_token,
        mint::authority = susdu_token,
        mint::decimals = decimals,
        mint::token_program = token_program,
        extensions::metadata_pointer::authority = susdu_token,
        extensions::metadata_pointer::metadata_address = susdu_token,
        extensions::permanent_delegate::delegate = susdu_token,
    )]
    pub susdu_token: InterfaceAccount<'info, Mint>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_create_susdu(ctx: Context<CreateSusdu>, decimals: u8) -> Result<()> {
    require!(!ctx.accounts.susdu_config.is_susdu_token_initialized, SusduError::ConfigAlreadySetupSusdu);

    ctx.accounts.susdu_config.susdu_token = ctx.accounts.susdu_token.key();
    ctx.accounts.susdu_config.susdu_token_bump = ctx.bumps.susdu_token;
    ctx.accounts.susdu_config.is_susdu_token_initialized = true;
    ctx.accounts.susdu_config.total_supply = 0;
    
    let name = "SUSDU - Program Controlled Token".to_string();
    let symbol = "SUSDU".to_string();
    let uri = "https://bafybeib5rbwqc5hj52hhc6k6g4c5qfhlq2jkkeujypc3okvm7dqoypgcku.ipfs.w3s.link/usdu.png".to_string();

    // create metadata
    let susdu_token_bump = &[ctx.bumps.susdu_token];
    let seeds = &[
        &[
            SUSDU_SEED,
            susdu_token_bump,
        ][..],
    ];
    token_metadata_initialize(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataInitialize {
                token_program_id: ctx.accounts.token_program.to_account_info(),
                mint: ctx.accounts.susdu_token.to_account_info(),
                metadata: ctx.accounts.susdu_token.to_account_info(),
                mint_authority: ctx.accounts.susdu_token.to_account_info(),
                update_authority: ctx.accounts.susdu_token.to_account_info(),
            },
            seeds,
        ),
        name.clone(),
        symbol.clone(),
        uri.clone()
    )?;

    ctx.accounts.susdu_token.reload()?;
    let susdu_token_account = &mut ctx.accounts.susdu_token.to_account_info();
    let metadata = get_mint_with_len_extension::<TokenMetadata>(susdu_token_account)?;
    assert_eq!(metadata.mint, ctx.accounts.susdu_token.key());
    assert_eq!(metadata.name, name);
    assert_eq!(metadata.symbol, symbol);
    assert_eq!(metadata.uri, uri);

    let delegate = get_mint_with_permanent_delegate(susdu_token_account)?;
    assert_eq!(delegate.delegate, OptionalNonZeroPubkey::try_from(Some(ctx.accounts.susdu_token.key()))?);

    // transfer rent to susdu_token
    let extra_lamports = Rent::get()?.minimum_balance(susdu_token_account.data_len()) - susdu_token_account.lamports();
    if extra_lamports > 0 {
        invoke(
            &transfer(
                &ctx.accounts.admin.key(),
                &ctx.accounts.susdu_token.key(),
                extra_lamports,
            ),
            &[
                ctx.accounts.admin.to_account_info(),
                ctx.accounts.susdu_token.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    }

    emit!(SusduTokenCreated {
        susdu_token: ctx.accounts.susdu_token.key(),
        decimals,
    });
    Ok(())
}
