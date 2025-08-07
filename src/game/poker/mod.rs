mod game_player;
mod blind;
mod error;
mod hand;

use crate::game::Game;
use crate::game::poker::error::PokerGameError;
use crate::game::poker::game_player::PokerGamePlayer;
use crate::player::Player;

pub struct PokerGame {
    pub players: Vec<PokerGamePlayer>,
    pub seats: u32,
    pub default_buy_in_value: u32,
    pub buy_in_limit: u32,
    pub blind_price: u32,
}

pub struct CreatePokerGame {
    pub seats: u32,
    pub default_buy_in_value: u32,
    pub buy_in_limit: u32,
    pub blind_price: u32,
}

impl PokerGame {
    pub fn new(params: CreatePokerGame) -> Self {
        PokerGame {
            players: vec![],
            seats: params.seats,
            default_buy_in_value: params.default_buy_in_value,
            buy_in_limit: params.buy_in_limit,
            blind_price: params.blind_price
        }
    }

    pub fn add_player(&mut self, player: Player, seat: u32) -> Result<(), PokerGameError> {
        return self.validate_seat(seat).or_else(|e| return Err(e))
            .and_then(|_| self.validate_new_player(&player))
            .or_else(|e| return Err(e))
            .and_then(|_| self.handle_new_player(player, seat));
    }

    pub fn handle_new_player(&mut self, player: Player, seat:u32) -> Result<(), PokerGameError> {
        let mut poker_player = PokerGamePlayer::create(seat, player);

        return poker_player.fund(self.default_buy_in_value)
            .and_then(
                |_| {
                    self.players.push(poker_player);
                    Ok(())
                }
            ).map_err(|e| PokerGameError::SeatMustBeGreaterThanZero)
    }

    fn validate_new_player(&self, player: &Player) -> Result<(), PokerGameError> {
        let player_already_exist = self.players.iter().any(|p| p.player.id == player.id);

        if player_already_exist { return Err(PokerGameError::YouAreAlreadyAtTheGame()); };

        let name_already_in_use = self.players.iter().any(|p| p.player.name == player.name);

        if name_already_in_use {
            let player_name = self.players.iter().find(|p| p.player.name == player.name).unwrap();
            return Err(PokerGameError::NameAlreadyUsed(String::from(&player_name.player.name)))
        }

        Ok(())

    }

    pub fn validate_seat(&self, seat: u32) -> Result<(), PokerGameError> {
        if seat <= 0 {
            return Err(PokerGameError::SeatMustBeGreaterThanZero);
        }

        if seat > self.seats {
            return Err(PokerGameError::SeatGreaterThanAllowed(seat, self.seats));
        }

        if self.players.is_empty() {
            return Ok(())
        }

        let seat_already_used = self.players.iter().any(|p| p.seat == seat);

        if seat_already_used {
            return Err(PokerGameError::SeatAlreadyInUse);
        }

        Ok(())

    }



}

#[cfg(test)]
mod tests {
    use crate::game::poker::blind::Blind;
    use super::*;


    fn dummy_player_with_seat(seat: u32) -> PokerGamePlayer {
        PokerGamePlayer::create(seat, Player::create(1, "A", "ex.jpg"))
    }

    #[test]
    fn test_seat_must_be_greater_than_zero() {
        let game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:0,
            blind_price: 50
        });
        let result = game.validate_seat(0);
        assert!(matches!(result, Err(PokerGameError::SeatMustBeGreaterThanZero)));
    }

    #[test]
    fn test_seat_greater_than_allowed() {
        let game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });
        let result = game.validate_seat(6);
        assert!(matches!(result, Err(PokerGameError::SeatGreaterThanAllowed(6, 5))));
    }

    #[test]
    fn test_seat_valid_when_players_empty() {
        let game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });;
        let result = game.validate_seat(3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_seat_already_in_use() {
        let player = dummy_player_with_seat(2);
        let game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });;
        let result = game.validate_seat(2);
        assert!(matches!(result, Err(PokerGameError::SeatAlreadyInUse)));
    }

    #[test]
    fn test_seat_valid_and_not_taken() {
        let player = dummy_player_with_seat(2);
        let game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });;
        let result = game.validate_seat(3);
        assert!(result.is_ok());
    }

    fn player_with_id(id: usize, name: &str) -> Player {
        Player::create(id, name, "example.png")
    }

    #[test]
    fn test_add_player_successfully() {
        let mut game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:200,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });;
        let player = player_with_id(1, "Alice");
        let result = game.add_player(player, 1);
        assert!(result.is_ok());
        assert_eq!(game.players.len(), 1);
        assert_eq!(game.players[0].seat, 1);
        assert_eq!(game.players[0].cash_amount, 200);
        assert_eq!(game.players[0].buy_ins, 1);
    }

    #[test]
    fn test_add_player_seat_already_in_use() {
        let mut game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });;
        let p1 = player_with_id(1, "Alice");
        let p2 = player_with_id(2, "Bob");

        game.add_player(p1, 1).unwrap();
        let result = game.add_player(p2, 1);
        assert!(matches!(result, Err(PokerGameError::SeatAlreadyInUse)));
    }

    #[test]
    fn test_add_player_seat_too_low() {
        let mut game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });;
        let player = player_with_id(1, "Alice");
        let result = game.add_player(player, 0);
        assert!(matches!(result, Err(PokerGameError::SeatMustBeGreaterThanZero)));
    }

    #[test]
    fn test_add_player_seat_too_high() {
        let mut game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });;
        let player = player_with_id(1, "Alice");
        let result = game.add_player(player, 6);
        assert!(matches!(result, Err(PokerGameError::SeatGreaterThanAllowed(6, 5))));
    }

    #[test]
    fn test_add_player_duplicate_id() {
        let mut game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });;
        let p1 = player_with_id(1, "Alice");
        let p2 = player_with_id(1, "Bob"); // Same ID, different name

        game.add_player(p1, 1).unwrap();
        let result = game.add_player(p2, 2);
        assert!(matches!(result, Err(PokerGameError::YouAreAlreadyAtTheGame())));
    }

    #[test]
    fn test_add_player_duplicate_name() {
        let mut game = PokerGame::new(CreatePokerGame{
            default_buy_in_value:0,
            buy_in_limit:2,
            seats:5,
            blind_price: 50
        });;
        let p1 = player_with_id(1, "Alice");
        let p2 = player_with_id(2, "Alice"); // Same name, different ID

        game.add_player(p1, 1).unwrap();
        let result = game.add_player(p2, 2);
        assert!(matches!(result, Err(PokerGameError::NameAlreadyUsed(name)) if name == "Alice"));
    }

}

