use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;

#[event]
pub struct AuthChangeEvent {
    pub authority: Pubkey,
}

#[event]
pub struct PriceSetEvent {
    pub token_addr: Pubkey,
    pub price: u64,
}

#[event]
pub struct PaymentAddEvent {
    pub token_addr: Pubkey,
    pub allow_num: u8,
}
