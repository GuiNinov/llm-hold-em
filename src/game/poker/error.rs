use crate::deck::Card;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerGameError {
    SeatAlreadyInUse,
    SeatMustBeGreaterThanZero,
    SeatGreaterThanAllowed(i32, i32),
    YouAreAlreadyAtTheGame(),
    NameAlreadyUsed(String),
    CardAlreadyInHand(Card),
    CannotFundNonGreaterThanZeroValues(i32)
}