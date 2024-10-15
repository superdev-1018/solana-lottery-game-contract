use anchor_lang::prelude::*;

#[error_code] 
pub enum ContractError {
    #[msg("Invalid Owner")]
    NotOwner,
    #[msg("Invalid address.")]
    InvalidAddress,
    #[msg("Lottery not started")]
    LotteryNotStarted,
    #[msg("Lottery ended")]
    LotteryEnded,
    #[msg("Already participated")]
    AlreadyParticipated,
    #[msg("Lottery not ended")]
    LotteryNotEnded,
    #[msg("Lottery not founded")]
    LotteryNotFound,
    #[msg("Lottery already ended")]
    LotteryAlreadyEnded,
    #[msg("There is no spot")]
    LotteryAlreadyFulled,
    #[msg("Invalid Mint Authority")]
    InvalidMintAuthority,
    #[msg("Referral Link Already Exist")]
    ReferralLinkAlreadyExist,
    #[msg("Referral Link MisMatched")]
    ReferralLinkMisMatched,
    #[msg("Not Enough Participants")]
    NotEnoughParticipants,
    #[msg("Invalid User Account")]
    InvalidUserAccount,
    #[msg("Lottery is in progress")]
    StillInProgress,
}