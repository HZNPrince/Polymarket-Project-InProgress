use anchor_lang::prelude::{pubkey::declare_id, *};

declare_id!("FkmMuomRKMLuuXFr5F63K3eng2DENDEnSQE2dv6Yh9og");

#[program]
pub mod resolution {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
