use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, Token2022, Burn, burn, TransferChecked, transfer_checked};
use anchor_spl::associated_token::get_associated_token_address_with_program_id;

use crate::constants::{SUSDU_CONFIG_SEED, SUSDU_SEED};
use crate::error::SusduError;
use crate::state::SusduConfig;

use guardian::utils::has_role;
use guardian::state::{AccessRegistry, AccessRole, Role};
use guardian::constants::{ACCESS_REGISTRY_SEED, ACCESS_ROLE_SEED};

#[derive(Accounts)]
pub struct RedistributeSusdu<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [SUSDU_CONFIG_SEED],
        bump = susdu_config.bump,
    )]
    pub susdu_config: Box<Account<'info, SusduConfig>>,
    #[account(mut)]
    pub susdu_token: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub locked_susdu_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    
    /// CHECK: will be checked in the process function
    #[account(mut)]
    pub receiver_susdu_token_account: UncheckedAccount<'info>,

    // Access Registry
    #[account(
        seeds = [ACCESS_REGISTRY_SEED],
        seeds::program = guardian::id(),
        bump = access_registry.bump,
    )]
    pub access_registry: Box<Account<'info, AccessRegistry>>,
    #[account(
        seeds = [
            ACCESS_ROLE_SEED,
            access_registry.key().as_ref(),
            authority.key().as_ref(),
            Role::SusduRedistributor.to_seed().as_slice(),
        ],
        bump = access_role.bump,
        seeds::program = guardian::id(),
    )]
    pub access_role: Box<Account<'info, AccessRole>>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn process_redistribute_susdu(ctx: Context<RedistributeSusdu>, receiver: Pubkey, amount: u64) -> Result<()> {
    require!(
        has_role(
            &ctx.accounts.access_registry,
            &ctx.accounts.access_role,
            &ctx.accounts.authority,
            Role::SusduRedistributor,
        )?,
        SusduError::UnauthorizedRole
    );

    let signer_seeds: &[&[&[u8]]] = &[&[SUSDU_SEED, &[ctx.accounts.susdu_config.susdu_token_bump]]];
    let susdu_config = &mut ctx.accounts.susdu_config;
    if ctx.accounts.receiver_susdu_token_account.key() == Pubkey::default() {
        // burn locked susdu token account
        burn(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.susdu_token.to_account_info(),
                    from: ctx.accounts.locked_susdu_token_account.to_account_info(),
                    authority: ctx.accounts.susdu_token.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;
        susdu_config.total_supply -= amount;
    } else {
        // transfer susdu token from locked susdu token account to receiver_susdu_token_account
        let ata = get_associated_token_address_with_program_id(&receiver, &ctx.accounts.susdu_token.key(), &ctx.accounts.token_program.key());
        msg!("ata: {}", ata);
        msg!("receiver_susdu_token_account: {}", ctx.accounts.receiver_susdu_token_account.key());
        require!(ata == ctx.accounts.receiver_susdu_token_account.key(), SusduError::InvalidReceiverAssociatedTokenAddress);
        transfer_checked(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.locked_susdu_token_account.to_account_info(),
                    to: ctx.accounts.receiver_susdu_token_account.to_account_info(),
                    authority: ctx.accounts.susdu_token.to_account_info(),
                    mint: ctx.accounts.susdu_token.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            ctx.accounts.susdu_token.decimals,
        )?;
    }

    Ok(())
}
