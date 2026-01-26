use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Offer {
    // Details of the offer made, e.g. what who made it and what they want in return.

    // identifier of the struct
    pub id: u64,

    // who make offer
    pub maker: Pubkey,

    // token being offered
    pub token_mint_a: Pubkey,

    // token wants in return
    pub token_mint_b: Pubkey,

    // how much want
    pub token_b_wanted_amount: u64,

    pub bump: u8
}
