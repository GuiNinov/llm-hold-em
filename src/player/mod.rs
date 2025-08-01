use crate::deck::Card;

#[derive(Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn create(id: usize, name: &str) -> Self {
        Player {
            id,
            name: name.to_string(),
            hand: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::deck::{Card, Rank, Suit};

    fn sample_card() -> Card {
        Card { rank: Rank::Ace, suit: Suit::Spades }
    }

    fn second_card() -> Card {
        Card { rank: Rank::King, suit: Suit::Hearts }
    }

    #[test]
    fn test_create_player() {
        let player = Player::create(1, "J");
        assert_eq!(player.id, 1);
        assert_eq!(player.name, "J");
        assert_eq!(player.hand.len(), 0);
    }
}
