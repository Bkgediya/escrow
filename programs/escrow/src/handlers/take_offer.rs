use anchor_lang::prelude::*;
use crate::{error::ErrorCode, handlers::{close_token_account, transfer_tokens}, state::{Offer, offer}};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint,TokenAccount,TokenInterface}
};

#[derive(Accounts)]
pub struct TakeOffer<'info> {  
    // TakeOffer (in capitals) is a struct of names accounts that the
    // take_offer() function will use.
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program: Program<'info,System>,
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(mut)]
    pub maker: SystemAccount<'info>,

    pub token_mint_a: InterfaceAccount<'info,Mint>,
    pub token_mint_b: InterfaceAccount<'info,Mint>,

    
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program

    )]
    pub taker_token_account_a : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = token_mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_token_account_b : InterfaceAccount<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = token_mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_token_account_b : InterfaceAccount<'info,TokenAccount>,


    #[account(
        mut,
        close = maker,
        has_one = maker,
        has_one = token_mint_b,
        seeds = [b"offer",offer.id.to_le_bytes().as_ref()],
        bump = offer.bump,
    )]
    pub offer : Account<'info,Offer>,

    #[account(
        mut,
        associated_token::mint = token_mint_a,
        associated_token::authority = offer,
        associated_token::token_program = token_program
    )]
    pub vault : InterfaceAccount<'info,TokenAccount>,
}

// Handle the take offer instruction by:
// 1. Sending the wanted tokens from the taker to the maker
// 2. Withdrawing the offered tokens from the vault to the taker and closing the vault
pub fn take_offer(_context: Context<TakeOffer>) -> Result<()> {
    let offer_account_seeds: &[&[u8]]  = &[
        b"offer",
        &_context.accounts.offer.id.to_le_bytes()[..],
        &[_context.accounts.offer.bump],
    ];
    
    let signers_seeds = Some(&offer_account_seeds[..]);

    transfer_tokens(
        &_context.accounts.vault,
        &_context.accounts.taker_token_account_a,
        &_context.accounts.vault.amount,
        &_context.accounts.token_mint_a,
        &_context.accounts.offer.to_account_info(),
        &_context.accounts.token_program,
        signers_seeds
    ).map(|_| ErrorCode::FailedVaultWithdrawal)?;


    close_token_account(
        &_context.accounts.vault,
        &_context.accounts.taker.to_account_info(),
        &_context.accounts.offer.to_account_info(),
        &_context.accounts.token_program,
        signers_seeds
    ).map(|_| ErrorCode::FailedVaultClosure)?;


    transfer_tokens(
        &_context.accounts.taker_token_account_b,
        &_context.accounts.maker_token_account_b,
        &_context.accounts.offer.token_b_wanted_amount,
        &_context.accounts.token_mint_b,
        &&_context.accounts.taker.to_account_info(),
        &_context.accounts.token_program,
        signers_seeds
    ).map(|_| ErrorCode::FailedTokenTransfer)?;

    Ok(())
}
