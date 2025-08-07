use std::ops::Index;
use crate::deck::{Card, Deck};
use crate::game::poker::error::PokerGameError;
use crate::game::poker::game_player::PokerGamePlayer;
use crate::player::Player;

pub enum HandStage {
    INIT,
    FLOP,
    RIVER,
    TURN
}

struct Fold {
    player: PokerGamePlayer,
    at: HandStage
}

struct PokerHand {
    pot: u32,
    flop: [Card; 3],
    river: Card,
    turn: Card,
    dealer_seat: u32,
    small_blind_seat: u32,
    big_blind_seat: u32,
    blind_price: u32,
    speaker: PokerGamePlayer,
    folds: Vec<Fold>,
    active: Vec<PokerGamePlayer>
}

struct StartHandParams {
    pub blind_price: u32,
    pub players: Vec<PokerGamePlayer>,
    pub dealer_seat: u32,
}

impl PokerHand {
    fn start(params: StartHandParams) -> Result<PokerHand, PokerGameError>  {
        let mut deck = Deck::create();
        let players = params.players;
        let dealer_seat = params.dealer_seat;

        let sorted_players_result = PokerHand::sort_players(&players, dealer_seat);
        if sorted_players_result.is_err() {
            return Err(sorted_players_result.unwrap_err());
        }

        let mut sorted_players = sorted_players_result.expect("Expect to be ok");
        for _ in 0..2 {
            for player in sorted_players.iter_mut() {
                let next_card = deck.get_next_card();
                player.receive_card(next_card).expect("Expect to receive the card");
            }

            deck.get_next_card(); // Burn the next card
        };

        let small_blind_player = &sorted_players[1];
        let big_blind_player = &sorted_players[2];





        // return PokerHand {
        //
        // }
    }


    fn sort_players(players: &Vec<PokerGamePlayer>, dealer_seat: u32) -> Result<Vec<PokerGamePlayer>, PokerGameError> {
        let mut sorted_players = players.clone();

        sorted_players.sort_by(|a, b| a.seat.cmp(&b.seat));

        let player_index = sorted_players.iter().position(|p| p.seat == dealer_seat);

        if player_index.is_none(){
            return Err(PokerGameError::PlayerSeatNotFound(dealer_seat))
        }

        sorted_players.rotate_left(player_index.expect("Expect to have index") as usize);

        Ok(sorted_players)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_players() {
        let players = vec![
            PokerGamePlayer::create(
                1, Player::create(1, "G", "l.png")
            ),
            PokerGamePlayer::create(
                2, Player::create(2, "P", "l.png")
            ),
            PokerGamePlayer::create(
                3, Player::create(3, "P", "l.png")
            ),
            PokerGamePlayer::create(
                4, Player::create(4, "P", "l.png")
            ),
            PokerGamePlayer::create(
                5, Player::create(5, "P", "l.png")
            )
        ];
        let sorted_players = PokerHand::sort_players(&players, 1).unwrap();
        assert_eq!(sorted_players[0].seat, 1);

        let updated_sorted_players = PokerHand::sort_players(&players, 2).unwrap();
        assert_eq!(updated_sorted_players[0].seat, 2);

        let updated_sorted_players = PokerHand::sort_players(&players, 3).unwrap();
        assert_eq!(updated_sorted_players[0].seat, 3);

        let updated_sorted_players = PokerHand::sort_players(&players, 4).unwrap();
        assert_eq!(updated_sorted_players[0].seat, 4);

        let updated_sorted_players = PokerHand::sort_players(&players, 5).unwrap();
        assert_eq!(updated_sorted_players[0].seat, 5);

    }

    #[test]
    fn test_sort_with_empty_seats() {
        let players = vec![
            PokerGamePlayer::create(
                1, Player::create(1, "G", "l.png")
            ),
            PokerGamePlayer::create(
                3, Player::create(3, "P", "l.png")
            ),
            PokerGamePlayer::create(
                5, Player::create(5, "T", "l.png")
            )
        ];
        let sorted_players = PokerHand::sort_players(&players, 1).unwrap();
        assert_eq!(sorted_players[0].seat, 1);

        let updated_sorted_players = PokerHand::sort_players(&players, 3).unwrap();
        assert_eq!(updated_sorted_players[0].seat, 3);

        let updated_sorted_players = PokerHand::sort_players(&players, 5).unwrap();
        assert_eq!(updated_sorted_players[0].seat, 5);
    }

}
