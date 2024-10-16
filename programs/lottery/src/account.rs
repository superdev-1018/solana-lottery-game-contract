use anchor_lang::prelude::*;
pub use crate::error::*;
use crate::MAX_PLAYERS;

#[account]
#[derive(Default)]
pub struct GlobalAccount {
    pub initializer: Pubkey,
    pub is_initialized: u8,
    // pub pool_toke_account: Pubkey,
    // pub withdraw_token_account: Pubkey
}

#[account]
pub struct User {
    pub id: Pubkey,
    pub spot: [u8;10],
    pub referral_link: String,
    pub referrer: Pubkey,
    pub referral_list: Vec<Pubkey>,
}


#[account]
pub struct Lottery{
    pub id: u8,
    pub time_frame: u64,
    pub ticket_price: u8,
    pub max_ticket: u64,
    pub dev_fee: u32,
    pub start_time: i64,
    pub end_time: i64,
    pub state: u8,
    pub participants: Vec<Pubkey>,
    pub winner: [Pubkey; 3],
    pub prize_percent: [u8; 3],
    pub winner_prize:[u64;3],
    pub real_pool_amount: u64,
    pub real_count: u32,
    pub round: u32
}

impl Lottery {
    pub const SIZE: usize = 202 + 32 * MAX_PLAYERS;
}

#[account]
pub struct LotteryPdaInfo {
    pub count: u8,
    pub rounds: [u8;10]
}

#[account]
pub struct WinnerTicker {
    pub winner: Pubkey,
    pub time_frame: u64,
    pub prize: u64 
}

#[account]
pub struct DepositeTicker {
    pub depositer: Pubkey,
    pub time_frame: u64,
    pub spots: u8,
    pub amount: u64
}



