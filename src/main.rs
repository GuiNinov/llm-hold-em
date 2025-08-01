
use crate::deck::Deck;
use crate::player::Player;
mod deck;
mod player;
mod game;

fn main() {
    let mut deck = Deck::create();
    deck.shuffle();

    let mut players = vec![
        Player::create(1, "AggressiveBot"),
        Player::create(2, "CautiousGPT"),
    ];

    const CARDS_PER_HAND: usize = 2;

    for _ in 0..CARDS_PER_HAND {
        for player in players.iter_mut() {
            let next_card = deck.get_next_card();
            // player.receive_card(next_card).unwrap()
        }

        deck.get_next_card();
    }

    for player in players.iter_mut() {
        // player.show_hand()
    }
}
