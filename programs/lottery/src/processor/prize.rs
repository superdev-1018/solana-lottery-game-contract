use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
pub use crate::{account::*, constant::*, error::*};

#[derive(Accounts)]
pub struct PrizeDistribute<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    #[account(mut)]
    pub winner1_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub winner2_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub winner3_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn send_prize(ctx: Context<PrizeDistribute>) -> Result<()> {

    let lottery = &mut ctx.accounts.lottery;

    let winner1_prize = lottery.winner_prize[0] * 1_000_000_000u64;
    let winner2_prize = lottery.winner_prize[1] * 1_000_000_000u64;
    let winner3_prize = lottery.winner_prize[2] * 1_000_000_000u64;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.winner1_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info()
            }
        ),
        winner1_prize
    )?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.winner2_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info()
            }
        ),
        winner2_prize
    )?;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.winner3_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info()
            }
        ),
        winner3_prize
    )?;

    Ok(())
}