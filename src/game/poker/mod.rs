mod game_player;
mod blind;

use crate::deck::Deck;
use crate::game::Game;
use crate::game::poker::game_player::PokerGamePlayer;
use crate::player::Player;

pub struct PokerGame {
    pub players: Vec<PokerGamePlayer>,
    pub deck: Deck,
}
impl PokerGame {
    pub fn new(player_names: Vec<&str>) -> Self {
        let players = player_names
            .into_iter()
            .enumerate()
            .map(|(i, name)| PokerGamePlayer::create(
                i32::try_from(i).unwrap(),
                Player::create(i, name)
            ))
            .collect();

        let mut deck = Deck::create();
        deck.shuffle();

        PokerGame { players, deck }
    }
}

impl Game for PokerGame {
    fn name(&self) -> &str {
        &"Poker"
    }

    fn start(&mut self) {
        todo!()
    }

    fn play_round(&mut self) {
        todo!()
    }

    fn show_players(&self) {
        todo!()
    }

    fn add_new_player(&mut self, player: Player) {
        todo!()
    }
}
