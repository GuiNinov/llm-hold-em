mod poker;

use crate::player::Player;

pub trait Game {
    fn name(&self) -> &str;
    fn start(&mut self);
    fn play_round(&mut self);
    fn show_players(&self);
    fn add_new_player(&mut self, player: Player);
}
