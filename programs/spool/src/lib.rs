use anchor_lang::prelude::*;

declare_id!("6jh18NDLTASsVvoZYSGwY3n9nDWmY8bVLAQhoYphuXTs");

#[program]
pub mod spool {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
