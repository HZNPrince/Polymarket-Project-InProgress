use anchor_lang::prelude::{pubkey::declare_id, *};

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod constants;
pub use constants::*;

declare_id!("C22aRjpwwUzmgcnHYs54rvd6EP1JT6uHfwcczRDZnhLK");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize_protocol(ctx: Context<InitProtocol>, fee_bps: u16) -> Result<()> {
        process_initialize_protocol(ctx, fee_bps)?;

        Ok(())
    }
}
