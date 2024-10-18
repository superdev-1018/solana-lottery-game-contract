use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
pub use crate::{account::*, constant::*, error::*};

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub global_account: Box<Account<'info, GlobalAccount>>,

    #[account(mut)]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub buyer_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = buyer, 
        seeds = [USER_INFO, buyer.key().as_ref()], 
        bump, 
        space = 8 + std::mem::size_of::<User>()
    )]
    pub user: Box<Account<'info, User>>,

    // #[account(mut, seeds = [LOTTERY_INFO, admin_key.as_ref(), &id.to_le_bytes()],bump,)]
    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    #[account(mut)]
    pub deposite_ticker: Box<Account<'info, DepositeTicker>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct GetUserTicket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub user: Box<Account<'info, User>>,

    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>> 
}

pub fn getticket(ctx: Context<BuyTicket>, count:u8) -> Result<()> {

    let lottery = &mut ctx.accounts.lottery;
    let buyer = &ctx.accounts.buyer;
    let user =&mut ctx.accounts.user;
    let transfer_amount = (lottery.ticket_price as u64) * (count as u64) * 1_000_000_000u64; 

    msg!("transfer token amount {}", transfer_amount);
    msg!("Buyer token account owner: {:?}", ctx.accounts.buyer_token_account.owner);
    msg!("Authority for transfer: {:?}", ctx.accounts.buyer.key);


    let transfer_instruction = Transfer {
        from: ctx.accounts.buyer_token_account.to_account_info(),
        to: ctx.accounts.pool_token_account.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let _ = anchor_spl::token::transfer(CpiContext::new(cpi_program, transfer_instruction), transfer_amount)?;

    lottery.real_pool_amount += (lottery.ticket_price as u64) * 1_000_000_000u64; 
    user.id = buyer.key();
    let lottery_timeframe = lottery.time_frame;

    let time_frames = [1, 3, 6, 12, 24, 168, 720, 2160, 4320, 8640];
    if let Some(index) = time_frames.iter().position(|&timeframe| timeframe == lottery_timeframe) {
        user.spot[index] += count;
    }

    let deposite_ticker = &mut ctx.accounts.deposite_ticker;
    deposite_ticker.depositer = buyer.key();
    deposite_ticker.time_frame = lottery.time_frame;
    deposite_ticker.spots = count;
    deposite_ticker.amount = (lottery.ticket_price * count) as u64;
    
    Ok(()) 
}



pub fn get_user_ticket_num(ctx: Context<GetUserTicket>) -> Result<bool> {
    let user = &ctx.accounts.user;
    let lottery = &ctx.accounts.lottery;
    let time_frames = [1, 3, 6, 12, 24, 168, 720, 2160, 4320, 8640];
    let lottery_timeframe = lottery.time_frame;
    let user_spot = user.spot;
    if let Some(index) = time_frames.iter().position(|&timeframe| timeframe == lottery_timeframe) {
        let spot = user_spot[index];
        msg!("user ticket {}", spot);
        if spot as usize > 0 {
            msg!("User has {} tickets", spot);
            return Ok(true); 
        } else {

            msg!("User does not have tickets or there is no lottery for {}", lottery_timeframe);
            return Ok(false);
        }

    }

    return Ok(false);
}


