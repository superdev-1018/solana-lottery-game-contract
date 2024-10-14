use anchor_lang::prelude::*;

declare_id!("HeuxH7NSWL4DFwV8qcTJKQvLp4PHEp5jjsUVBLYw8rz1");

pub mod constant;
pub mod error;
pub mod processor;
pub mod account;

pub use constant::*;
pub use processor::*;


#[program]
pub mod lottery {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
      msg!("This is entry to {:?}", "Initialize");
       initialize::init(ctx)
    }

    pub fn create_lottery(ctx: Context<CreateLottery>, id:u8, time_frame_index:u8, time_frame:u64, ticket_price: u8, max_ticket:u64, dev_fee: u32) -> Result<()> {
      msg!("create lottery {}", time_frame_index);
      lottery::create(ctx, id, time_frame_index, time_frame, ticket_price, max_ticket, dev_fee)
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, count:u8) -> Result<()> {
      msg!("buy ticket {}", "id");
      user::getticket(ctx, count)
    }

    pub fn end_lottery(ctx: Context<EndLottery>) -> Result<()> {
      msg!("end lottery {}", "id");
      lottery::endlottery(ctx)
    }

    pub fn prize_distribution(ctx: Context<PrizeDistribute>) -> Result<()> {
      msg!("send prize {}", "id");
      lottery::send_prize(ctx)
    }

    pub fn set_referral(ctx: Context<SetReferralLink>, referral_link:String) -> Result<()> {
      msg!("set user referral link {}", "id");
      referral::setreferral(ctx, referral_link)
    }

    pub fn add_referral_user(ctx: Context<AddReferralUser>) -> Result<()> {
      msg!("add referral user {}", "id");
      referral::add_referral_user(ctx)
    }

    pub fn get_user_ticket(ctx: Context<GetUserTicket>) -> Result<bool> {
      msg!("get user ticket {}", "id");
      user::get_user_ticket_num(ctx)
    }

    pub fn join_lottery(ctx: Context<JoinLottery>) -> Result<()> {
      msg!("join lottery {}", "id");
      lottery::join_to_lottery(ctx)
    }

}

// fn check_owner<'info>(
//   global_account: &Account<'info, GlobalAccount>,
//   signer: &Signer<'info>
// ) -> Result<()> {
//   require!(signer.key() == global_account.initializer || signer.key() == global_account.withdrawer, ContractError::NotOwner);
//   Ok(())
// }
