
use rand::seq::SliceRandom;
use rand::{rng};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two = 2, Three, Four, Five, Six, Seven, Eight,
    Nine, Ten, Jack, Queen, King, Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank = match self.rank {
            Rank::Two => "2", Rank::Three => "3", Rank::Four => "4", Rank::Five => "5",
            Rank::Six => "6", Rank::Seven => "7", Rank::Eight => "8", Rank::Nine => "9",
            Rank::Ten => "10", Rank::Jack => "J", Rank::Queen => "Q", Rank::King => "K",
            Rank::Ace => "A"
        };
        let suit = match self.suit {
            Suit::Hearts => "♥", Suit::Diamonds => "♦", Suit::Clubs => "♣", Suit::Spades => "♠"
        };
        write!(f, "{}{}", rank, suit)
    }
}

#[derive(Debug)]
pub enum DeckError {
    ShuffleRoundsMustBeGreaterThanZero, 
}

impl Deck {
    pub fn create() -> Deck {
        let mut deck = Vec::with_capacity(52);
        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for rank in 2..=14 {
                let rank = match rank {
                    2 => Rank::Two, 3 => Rank::Three, 4 => Rank::Four, 5 => Rank::Five,
                    6 => Rank::Six, 7 => Rank::Seven, 8 => Rank::Eight, 9 => Rank::Nine,
                    10 => Rank::Ten, 11 => Rank::Jack, 12 => Rank::Queen, 13 => Rank::King, _ => Rank::Ace,
                };
                deck.push(Card { rank, suit: *suit });
            }
        }

        Deck { cards: deck }
    }

    pub fn shuffle(&mut self, rounds: usize) -> Result<(), DeckError> {
        if(rounds <= 0) {
           return Err(DeckError::ShuffleRoundsMustBeGreaterThanZero)
        }

        for _ in 0..rounds {
            self.cards.shuffle(&mut rng());
        }

        Ok(())
    }

    pub fn get_next_card(&mut self) -> Card {
        let first_card = self.cards.first().cloned().unwrap();

        self.cards.remove(0);

        return first_card;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_deck_has_52_cards() {
        let deck = Deck::create();
        assert_eq!(deck.cards.len(), 52, "Deck should have 52 cards");
    }

    #[test]
    fn test_create_deck_has_unique_cards() {
        let deck = Deck::create();
        let mut seen = std::collections::HashSet::new();
        for card in &deck.cards {
            let key = format!("{:?}-{:?}", card.rank, card.suit);
            assert!(seen.insert(key), "Duplicate card found: {:?}", card);
        }
        assert_eq!(seen.len(), 52, "All cards in the deck should be unique");
    }

    #[test]
    fn test_shuffle_changes_order() {
        let mut deck1 = Deck::create();
        let mut deck2 = Deck::create();

        deck1.shuffle(1);

        // It is *possible* for two shuffled decks to have the same order, but highly unlikely
        let has_same_order = deck1.cards.iter().zip(deck2.cards.iter()).all(|(a, b)| a == b);
        assert!(!has_same_order, "Shuffled deck should likely not be in the same order as original");
    }

    #[test]
    fn test_suffle_negative_rounds() {
        let mut deck = Deck::create();

        let res = deck.shuffle(0);

        assert!(res.is_err());
    }

    #[test]
    fn test_get_next_card_reduces_deck_size() {
        let mut deck = Deck::create();
        let initial_len = deck.cards.len();
        let card = deck.get_next_card();
        assert_eq!(deck.cards.len(), initial_len - 1, "Deck should lose a card after drawing");
        assert!(!deck.cards.contains(&card), "Drawn card should no longer be in the deck");
    }

    #[test]
    fn test_get_next_card_returns_first_card() {
        let mut deck = Deck::create();
        let expected_card = deck.cards[0];
        let actual_card = deck.get_next_card();
        assert_eq!(expected_card, actual_card, "Should return and remove the first card");
    }

    #[test]
    fn test_get_next_card_two_times_should_return_different_cards() {
        let mut deck = Deck::create();
        let expected_card = deck.cards[1];

        deck.get_next_card();
        let actual_card = deck.get_next_card();

        assert_eq!(expected_card, actual_card, "Should return and remove the second card");
    }
}