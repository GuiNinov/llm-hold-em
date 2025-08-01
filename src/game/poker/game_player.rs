use crate::deck::Card;
use crate::game::poker::blind::Blind;
use crate::player::{Player};
use crate::game::poker::error::PokerGameError;

#[derive()]
pub struct PokerGamePlayer {
    pub seat: i32,
    pub player: Player,
    pub blind: Blind,
    pub cash_amount: i32,
    pub hand: Vec<Card>,
    pub buy_ins: i32,
}

impl PokerGamePlayer {
    pub fn create(seat: i32, player: Player) -> PokerGamePlayer {
         PokerGamePlayer {
            player,
            seat,
            blind: Blind::NONE,
            cash_amount: 0,
            hand: vec![],
            buy_ins: 0,
        }
    }

    pub fn fund(&mut self, amount: i32) -> Result<(), PokerGameError>  {
        if amount <= 0 { return Err(PokerGameError::CannotFundNonGreaterThanZeroValues(amount)); }
        self.cash_amount += amount;
        self.buy_ins += 1;
        Ok(())
    }

    pub fn show_hand(&self) {
        println!("🧠 {}: {} {}", self.player.name, self.hand[0], self.hand[1]);
    }

    pub fn receive_card(&mut self, card: Card) -> Result<(), PokerGameError> {
        if self.hand.is_empty() {
            self.hand.push(card);
            return Ok(())
        }

        // check if already exists
        let exists = self.hand.iter().any(|c| *c == card);

        if exists {
            return Err(PokerGameError::CardAlreadyInHand(card))
        }

        self.hand.push(card);

        Ok(())

    }
}

#[cfg(test)]
mod tests {
    use crate::deck::{Rank, Suit};
    use super::*;
    use crate::player::Player;
    use crate::game::poker::blind::Blind;

    fn sample_card() -> Card {
        Card { rank: Rank::Ace, suit: Suit::Spades }
    }

    fn second_card() -> Card {
        Card { rank: Rank::King, suit: Suit::Hearts }
    }

    #[test]
    fn test_create_poker_game_player_defaults() {
            let player = Player::create(1, "Alice", "example.png");
        let pgp = PokerGamePlayer::create(2, player);

        assert_eq!(pgp.seat, 2);
        assert_eq!(pgp.player.name, "Alice");
        assert_eq!(pgp.blind, Blind::NONE);
        assert_eq!(pgp.cash_amount, 0);
        assert_eq!(pgp.hand.len(), 0);
        assert_eq!(pgp.buy_ins, 0);
    }

    #[test]
    fn test_fund_adds_cash() {
        let player = Player::create(1, "Bob", "example.png");
        let mut pgp = PokerGamePlayer::create(3, player);

        pgp.fund(500)
            .and_then(|_| {
                assert_eq!(pgp.cash_amount, 500);
                assert_eq!(pgp.buy_ins, 1);
                Ok(())
            }).expect("TODO: panic message");


        pgp.fund(250)
            .and_then(|_| {
                assert_eq!(pgp.cash_amount, 750);
                assert_eq!(pgp.buy_ins, 2);
                Ok(())
            }).expect("TODO: panic message");
    }
    #[test]
    fn test_fund_zero_value() {
        let player = Player::create(1, "Bob", "example.png");
        let mut pgp = PokerGamePlayer::create(3, player);
        let res = pgp.fund(0);
        assert!(res.is_err());
        assert!(matches!(res.unwrap_err(), PokerGameError::CannotFundNonGreaterThanZeroValues(_card)));
    }

    #[test]
    fn test_receive_card_adds_to_hand() {
        let player = Player::create(1, "Alice", "example.png");
        let mut pgp = PokerGamePlayer::create(0, player);
        let card = sample_card();

        let result = pgp.receive_card(card);
        assert!(result.is_ok());
        assert_eq!(pgp.hand.len(), 1);
        assert_eq!(pgp.hand[0], card);
    }

    #[test]
    fn test_receive_two_different_cards() {
        let player = Player::create(2, "Bob", "example.png");
        let mut pgp = PokerGamePlayer::create(1, player);
        let card1 = sample_card();
        let card2 = second_card();

        assert!(pgp.receive_card(card1).is_ok());
        assert!(pgp.receive_card(card2).is_ok());

        assert_eq!(pgp.hand.len(), 2);
        assert!(pgp.hand.contains(&card1));
        assert!(pgp.hand.contains(&card2));
    }

    #[test]
    fn test_receive_duplicate_card_returns_error() {
        let player = Player::create(3, "Charlie", "example.png");
        let mut pgp = PokerGamePlayer::create(2, player);
        let card = sample_card();

        assert!(pgp.receive_card(card).is_ok());

        let result = pgp.receive_card(card);
        assert!(matches!(result, Err(PokerGameError::CardAlreadyInHand(_))));
    }
}
