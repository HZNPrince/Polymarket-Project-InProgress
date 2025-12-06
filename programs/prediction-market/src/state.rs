use anchor_lang::prelude::*;

#[derive(Clone, Copy, AnchorSerialize, AnchorDeserialize, InitSpace, PartialEq)]
pub enum MarketStatus {
    Active,
    Closed,
}

#[account]
#[derive(InitSpace)]
pub struct Protocol {
    pub admin: Pubkey,
    pub fee_bps: u16,
    pub treasury: Pubkey,
    pub market_count: u64,
    pub paused: bool,
    pub allowed_collateral: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub authority: Pubkey,
    #[max_len(100)]
    pub question: String,
    pub yes_mint: Pubkey,
    pub no_mint: Pubkey,
    pub yes_pool: u64,
    pub no_pool: u64,
    pub end_time: i64,
    pub oracle: Pubkey,
    pub collateral_vault: Pubkey,
    pub status: MarketStatus,
    pub outcome: Option<bool>,
    pub bump: u8,
}
