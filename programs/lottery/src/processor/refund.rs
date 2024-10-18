use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
pub use crate::{account::*, constant::*, error::*};

#[derive(Accounts)]
pub struct ReFund<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    #[account(mut)]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub participant_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn refund_ticket_price(ctx: Context<ReFund>) -> Result<()> {

    let lottery = &mut ctx.accounts.lottery;

    let ticket_price = (lottery.ticket_price as u64) * 1_000_000_000u64;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.participant_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info()
            }
        ),
        ticket_price
    )?;

    lottery.state = 0;

    Ok(())
}