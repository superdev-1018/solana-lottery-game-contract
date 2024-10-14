use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer};
pub use crate::error::*;
pub use crate::account::*;
pub use crate::constant::*;

#[derive(Accounts)]
pub struct SetReferralLink<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub user: Box<Account<'info, User>>,
}

#[derive(Accounts)]
pub struct AddReferralUser<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = buyer, 
        seeds = [USER_INFO, buyer.key().as_ref()], 
        bump, 
        space = 8 + std::mem::size_of::<User>())]
    pub referral_user: Box<Account<'info, User>>,

    #[account(mut)]
    pub referrer: Box<Account<'info, User>>,

    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    #[account(mut)]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub buyer_token_account: Box<Account<'info, TokenAccount>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}



pub fn setreferral(ctx: Context<SetReferralLink>, referral_link: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    require!(user.id == ctx.accounts.signer.key(), ContractError::InvalidUserAccount);
    if user.referral_link == referral_link {
        return Err(ContractError::ReferralLinkAlreadyExist.into());
    } else {
        user.referral_link = referral_link;
        Ok(())
    }
}

pub fn add_referral_user(ctx: Context<AddReferralUser>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let referrer = &mut ctx.accounts.referrer;
    let referral_user = &mut ctx.accounts.referral_user;
    referrer.referral_list.push(ctx.accounts.buyer.key());

    require!((lottery.state) !=2, ContractError::LotteryEnded);
    let max_tickets: usize = lottery.max_ticket.try_into().unwrap();
    require!(
        !lottery.participants.contains(ctx.accounts.buyer.key),
        ContractError::AlreadyParticipated
    );

    require!(
        lottery.participants.len() + 1 <= max_tickets,
        ContractError::LotteryAlreadyFulled
    );

    let transfer_amount = (lottery.ticket_price as u64) * 1_000_000_000u64;

    let transfer_instruction = Transfer {
        from: ctx.accounts.buyer_token_account.to_account_info(),
        to: ctx.accounts.pool_token_account.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let _ = anchor_spl::token::transfer(CpiContext::new(cpi_program, transfer_instruction), transfer_amount)?;
    
    let real_count = lottery.real_count;
    lottery.participants[real_count as usize] = *ctx.accounts.buyer.key;
    lottery.real_pool_amount += transfer_amount; 

    referral_user.id = *ctx.accounts.buyer.key;

    Ok(())
}
