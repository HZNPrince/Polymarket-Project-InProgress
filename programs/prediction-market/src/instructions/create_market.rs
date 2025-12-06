use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::TokenInterface;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::*;
use crate::state::*;

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub collateral_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = creator,
        space = 8 + Market::INIT_SPACE,
        seeds = [MARKET_SEEDS.as_bytes(), protocol.market_count.to_le_bytes().as_ref()],
        bump,
    )]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEEDS.as_bytes()],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,

    #[account(
        init,
        payer = creator,
        seeds = [YES_MINT.as_bytes(), market.key().as_ref()],
        bump,
        mint::authority = market,
        mint::decimals = 0,
        mint::token_program = token_program
    )]
    pub yes_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = creator,
        seeds = [NO_MINT.as_bytes(), market.key().as_ref()],
        bump,
        mint::authority = market,
        mint::decimals = 0,
        mint::token_program = token_program,
    )]
    pub no_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::authority = creator,
        associated_token::mint = collateral_mint,
        associated_token::token_program = token_program,
    )]
    pub creator_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = creator,
        seeds = [COLLATERAL_VAULT.as_bytes(), market.key().as_ref()],
        bump,
        token::authority = market,
        token::mint = collateral_mint,
        token::token_program = token_program,
    )]
    pub collateral_vault: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
