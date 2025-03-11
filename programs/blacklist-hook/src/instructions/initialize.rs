use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, Token2022};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::constants::{BLACKLIST_ENTRY_SEED, BLACKLIST_HOOK_CONFIG, EXTRA_ACCOUNT_META_LIST_SEED};
use crate::error::BlacklistHookError;
use crate::state::BlacklistHookConfig;

#[derive(Accounts)]
pub struct InitializeExtraAccountMeta<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK: ExtraAccountMetaList Account, must use this exact seeds
    #[account(
        mut,
        seeds=[EXTRA_ACCOUNT_META_LIST_SEED, mint.key().as_ref()],
        bump,
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [BLACKLIST_HOOK_CONFIG.as_bytes()],
        space = BlacklistHookConfig::SIZE,
        bump,
    )]
    pub blacklist_hook_config: Box<Account<'info, BlacklistHookConfig>>,
    /// CHECK: This is the mint account
    pub mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeExtraAccountMeta<'info> {
    pub fn extra_account_meta_list() -> Result<Vec<ExtraAccountMeta>> {
        Ok(vec![
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal {
                        bytes: BLACKLIST_ENTRY_SEED.as_bytes().to_vec(),
                    },
                    // source_token_account owner
                    Seed::AccountData {
                        account_index: 0,
                        data_index: 32,
                        length: 32,
                    },
                ],
                false, // is_signer
                false, // is_writable
            )?,
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal {
                        bytes: BLACKLIST_ENTRY_SEED.as_bytes().to_vec(),
                    },
                    // destination_token_account owner
                    Seed::AccountData {
                        account_index: 2,
                        data_index: 32,
                        length: 32,
                    },
                ],
                false, // is_signer
                false, // is_writable
            )?,
        ])
    }
}

pub fn process_initialize_extra_account_meta(
    ctx: Context<InitializeExtraAccountMeta>,
) -> Result<()> {
    let config = &mut ctx.accounts.blacklist_hook_config;
    let mint = &ctx.accounts.mint.key();
    // check if config is already initialized, avoid reinitializing
    require!(
        !config.is_initialized,
        BlacklistHookError::ConfigAlreadyInitialized
    );
    config.admin = ctx.accounts.admin.key();
    config.pending_admin = Pubkey::default();
    config.bump = ctx.bumps.blacklist_hook_config;
    config.is_initialized = true;

    let account_size = ExtraAccountMetaList::size_of(
        InitializeExtraAccountMeta::extra_account_meta_list()?.len(),
    )?;
    let lamports = Rent::get()?.minimum_balance(account_size);
    let signed_seeds: &[&[&[u8]]] = &[&[
        EXTRA_ACCOUNT_META_LIST_SEED,
        &mint.as_ref(),
        &[ctx.bumps.extra_account_meta_list],
    ]];

    create_account(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            CreateAccount {
                from: ctx.accounts.admin.to_account_info(),
                to: ctx.accounts.extra_account_meta_list.to_account_info(),
            },
            signed_seeds,
        ),
        lamports,
        account_size as u64,
        &crate::id(),
    )?;

    ExtraAccountMetaList::init::<ExecuteInstruction>(
        &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
        &InitializeExtraAccountMeta::extra_account_meta_list()?,
    )?;

    Ok(())
}
