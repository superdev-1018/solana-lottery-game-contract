use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
pub use crate::{account::*, constant::*, error::*};
use oorandom::Rand64;

#[derive(Accounts)]
#[instruction(id: u8)]
pub struct CreateLottery<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init, 
        payer = admin,
        seeds = [LOTTERY_INFO, admin.key().as_ref(), &id.to_le_bytes()],
        bump,
        space = 8 + std::mem::size_of::<Lottery>() 
    )]
    pub lottery: Box<Account<'info, Lottery>>,

    #[account(mut)]
    pub lottery_pdakey_info: Box<Account<'info, LotteryPdaInfo>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EndLottery<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    #[account(mut)]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub tax_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinLottery<'info> {
    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    #[account(mut)]
    pub user: Box<Account<'info, User>>,
}



pub fn create(ctx: Context<CreateLottery>, id:u8, time_frame_index:u8, time_frame:u64, ticket_price: u8, max_ticket:u64, dev_fee: u32) -> Result<()> {
    msg!("entrypoint of {}", "create");
    let lottery =&mut ctx.accounts.lottery;
    let current_time = Clock::get().unwrap().unix_timestamp;
    lottery.id = id;
    lottery.time_frame = time_frame;
    lottery.ticket_price = ticket_price;
    lottery.max_ticket = max_ticket;
    lottery.dev_fee = dev_fee;
    lottery.start_time = current_time;
    lottery.end_time = current_time + time_frame as i64;
    lottery.state = 0;
    lottery.real_pool_amount = 0;
    lottery.round +=1;
    lottery.real_count = 0;
    msg!("endpoint of {}","create");

    let lottery_pdakey_info = &mut ctx.accounts.lottery_pdakey_info;
    lottery_pdakey_info.count += 1;
    lottery_pdakey_info.rounds[time_frame_index as usize] += 1;
    Ok(())
}



pub fn endlottery(ctx: Context<EndLottery>) -> Result<()> {

    let lottery =&mut ctx.accounts.lottery;
    let participants = lottery.participants.len();
    let max_tickets: usize = lottery.max_ticket.try_into().unwrap();
    let is_in_progress = lottery.state == 1;

    require!(is_in_progress, ContractError::LotteryNotStarted);
    require!(participants > 3, ContractError::NotEnoughParticipants);
    require!(lottery.winner.len() == 0, ContractError::LotteryAlreadyEnded);

    let unique_numbers = [0,1,2];

    for i in 0..3 {
        let current_time: u128 = Clock::get().unwrap().unix_timestamp as u128;
        let mut rng1: Rand64 = Rand64::new(current_time);
        let winner_index: usize = rng1.rand_range(0..(participants as u64 -1) ).try_into().unwrap();
    }

    let winner_list: [u8; 3] = unique_numbers;

    // Collect winners' pubkeys
    let winners: Vec<Pubkey> = winner_list
        .iter()
        .filter_map(|&i| lottery.participants.get(i as usize).copied())
        .collect();

    let winner1= winners[0];
    let winner2 = winners[1];
    let winner3 = winners[2];
    lottery.winner = [winner1, winner2, winner3];

    // Calculate tax fee and update pool amount
    let lottery_pool_amount = lottery.real_pool_amount;
    let tax_fee = (lottery_pool_amount * 10 / 100) as u64;
    lottery.real_pool_amount -= tax_fee;

    // Transfer the tax fee
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.pool_token_account.to_account_info(),
                to: ctx.accounts.tax_token_account.to_account_info(),
                authority: ctx.accounts.admin.to_account_info(),
            },
        ),
        tax_fee,
    )?;

    let winner1_prize = lottery_pool_amount * 50/100 as u64;
    let winner2_prize = lottery_pool_amount * 30/100 as u64;
    let winner3_prize = lottery_pool_amount * 20/100 as u64;

    lottery.winner_prize = [winner1_prize, winner2_prize, winner3_prize];
    lottery.state = 2;

    Ok(())
}


pub fn join_to_lottery(ctx: Context<JoinLottery>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let lottery = &mut ctx.accounts.lottery;

    require!((lottery.state) !=2, ContractError::LotteryEnded);
    let max_tickets: usize = lottery.max_ticket.try_into().unwrap();

     require!(
        !lottery.participants.contains(&user.id),
        ContractError::AlreadyParticipated
    );

    require!(
        lottery.participants.len() + 1 <= max_tickets,
        ContractError::LotteryAlreadyFulled
    );

    let real_count = lottery.real_count;

    let transfer_amount = lottery.ticket_price as u64;
    lottery.participants[real_count as usize] = user.id;
    lottery.real_pool_amount += transfer_amount; 

    Ok(())
}