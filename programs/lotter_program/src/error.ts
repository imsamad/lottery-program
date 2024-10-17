use anchor_lang::prelude::error_code;

#[error_code]
pub enum LotteryError {
    #[msg("Winner already exists!")]
    WinnerAlreadyExists,

    #[msg("Can not choose a winner, no tickets!")]
    NoTickets,

    #[msg("Winner has not been choosen!")]
    WinnerNotChoosen,

    #[msg("Invalid winner")]
    InvalidWinner,

    #[msg("The prize has already been claimed!")]
    PrizeClaimedAlready,
}
