use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    constants::{PROTOCOL_SEEDS, TREASURY_SEEDS},
    Protocol,
};

#[derive(Accounts)]
pub struct InitProtocol<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + Protocol::INIT_SPACE,
        seeds = [PROTOCOL_SEEDS.as_bytes()],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,

    pub collateral_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = collateral_mint,
        associated_token::authority = protocol,
        associated_token::token_program = token_program,
    )]
    pub treasury: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn process_initialize_protocol(ctx: Context<InitProtocol>, fee_bps: u16) -> Result<()> {
    ctx.accounts.protocol.set_inner(Protocol {
        admin: ctx.accounts.signer.key(),
        fee_bps: fee_bps,
        treasury: ctx.accounts.treasury.key(),
        market_count: 0,
        paused: false,
        allowed_collateral: ctx.accounts.collateral_mint.key(),
    });

    Ok(())
}
