use anchor_lang::prelude::{pubkey::declare_id, *};

declare_id!("C22aRjpwwUzmgcnHYs54rvd6EP1JT6uHfwcczRDZnhLK");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
