use crate::deck::Card;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerGameError {
    SeatAlreadyInUse,
    SeatMustBeGreaterThanZero,
    SeatGreaterThanAllowed(u32, u32),
    YouAreAlreadyAtTheGame(),
    NameAlreadyUsed(String),
    CardAlreadyInHand(Card),
    CannotFundNonGreaterThanZeroValues(u32),
    PlayerSeatNotFound(u32)
}