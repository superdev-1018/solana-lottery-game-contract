use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
pub use crate::{account::*, constant::*, error::*};
use oorandom::Rand64;
use std::collections::HashSet;
use std::convert::TryInto;

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
        space = 8 + Lottery::SIZE 
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

    #[account(mut)]
    pub winner_ticker: Box<Account<'info, WinnerTicker>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,


}

#[derive(Accounts)]
pub struct JoinLottery<'info> {
    #[account(mut)]
    pub lottery: Box<Account<'info, Lottery>>,

    #[account(mut)]
    pub user: Box<Account<'info, User>>,

    pub system_program: Program<'info, System>,
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
    // require!(lottery.winner.len() == 0, ContractError::LotteryAlreadyEnded);
    require!(!lottery.participants.contains(ctx.accounts.system_program.key), ContractError::LotteryNotStarted);

    let mut unique_numbers = HashSet::new();
    let current_time: u128 = Clock::get().unwrap().unix_timestamp as u128;
    let mut rng1 = Rand64::new(current_time); 

    while unique_numbers.len() < 3 {
        let winner_index: usize = rng1.rand_range(0..participants as u64).try_into().unwrap();
        unique_numbers.insert(winner_index); 
    }

    let unique_numbers_vec: Vec<usize> = unique_numbers.into_iter().collect();
    let winner_list: [u8; 3];

    if unique_numbers_vec.len() == 3 {
        // Convert from Vec<usize> to [u8; 3]
        let unique_array: [u8; 3] = unique_numbers_vec
            .iter()
            .map(|&x| x as u8) // Convert usize to u8
            .collect::<Vec<u8>>() 
            .try_into() 
            .expect("Expected exactly 3 unique numbers");

        winner_list = unique_array; 
        msg!("Winners: {:?}", winner_list);
    } else {
        panic!("Not enough unique numbers generated");
    }


    // Collect winners' pubkeys
    let winners: Vec<Pubkey> = winner_list
        .iter()
        .filter_map(|&i| lottery.participants.get(i as usize).copied())
        .collect();

    let winner1= winners[0];
    let winner2 = winners[1];
    let winner3 = winners[2];
    lottery.winner = [winner1, winner2, winner3];
    msg!("winner list {}, {}, {}", winner1, winner2, winner3);
    // Calculate tax fee and update pool amount
    let lottery_pool_amount = lottery.real_pool_amount;
    let tax_fee = lottery_pool_amount * 10 / 100;
    lottery.real_pool_amount -= tax_fee;
    msg!("tax fee amount is {}", tax_fee);
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
    lottery.state = 1;

    let winner_ticker = &mut ctx.accounts.winner_ticker;
    winner_ticker.winner = winner1;
    winner_ticker.prize = lottery.real_pool_amount * 50;
    winner_ticker.time_frame = lottery.time_frame;

    Ok(())
}


pub fn join_to_lottery(ctx: Context<JoinLottery>, user_spot_index:u8) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let lottery = &mut ctx.accounts.lottery;

    require!((lottery.state) !=1, ContractError::LotteryEnded);
    let max_tickets: usize = lottery.max_ticket.try_into().unwrap();

     require!(
        !lottery.participants.contains(&user.id),
        ContractError::AlreadyParticipated
    );

    require!(
        lottery.participants.len() + 1 <= max_tickets,
        ContractError::LotteryAlreadyFulled
    );

    lottery.real_count += 1;

    let transfer_amount = lottery.ticket_price as u64;
    lottery.participants.push(user.id);
    lottery.real_pool_amount += transfer_amount; 
    user.spot[user_spot_index as usize] -= 1;

    Ok(())
}