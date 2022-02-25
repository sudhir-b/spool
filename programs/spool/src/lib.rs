use anchor_lang::prelude::*;
use anchor_spl::token::Token;

declare_id!("6jh18NDLTASsVvoZYSGwY3n9nDWmY8bVLAQhoYphuXTs");

#[program]
pub mod spool {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }

    pub fn draw(ctx: Context<Draw>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

const PROGRAM_STATE_SEED: &[u8] = b"programstate";

#[account]
#[derive(Default)]
pub struct ProgramState {
    pub total_staked_sol: u128,
}

#[account]
#[derive(Default)]
pub struct UserDeposit {
    pub staked_sol: u128,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        init_if_needed,
        payer = user,
        seeds = [&user.key().to_bytes()],
        bump,
    )]
    pub user_deposit: Account<'info, UserDeposit>,

    #[account(
        seeds = [PROGRAM_STATE_SEED],
        bump,
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw {}

#[derive(Accounts)]
pub struct Draw {}
