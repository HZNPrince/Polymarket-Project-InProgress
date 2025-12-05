use anchor_lang::prelude::{pubkey::declare_id, *};

declare_id!("5z7Fz1zXwY45JymVRTGQvNmXcegiz1zgwrGVdtLV5Lmn");

#[program]
pub mod market_factory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
