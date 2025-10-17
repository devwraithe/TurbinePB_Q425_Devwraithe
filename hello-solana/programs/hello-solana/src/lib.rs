#![allow(deprecated)] // Suppress deprecated warnings
#![allow(unexpected_cfgs)] // Suppress unrecognized cfg flag warnings

use anchor_lang::prelude::*;

declare_id!("2ZUoEbx1xXE6ok9Ln67ziRJLEhVQKoHS2z4i1Au4UVZL");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello Solana! 👋");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
