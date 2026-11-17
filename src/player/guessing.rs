use crate::interfaces::{GameState, Placement, Player, PlayerID};

#[allow(dead_code)]
pub struct GuessingPlayer {
    pub id: PlayerID,
}

impl GuessingPlayer {}

impl Player for GuessingPlayer {
    fn do_move(&mut self, board: &GameState) -> Placement {
        Placement::new_from_existing(board, 1.0_f32)
    }

    fn get_id(&self) -> PlayerID {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guessing_player_assigns_one_to_every_cell() {
        const N: u16 = 10;
        const ID: PlayerID = 1;
        let mut player = GuessingPlayer { id: ID };
        let board = GameState::new(N, N, None);
        let placement = player.do_move(&board);

        assert_eq!(placement.get_number_of_rows(), N);
        assert_eq!(placement.get_number_of_columns(), N);
        assert!(placement.iter_2d().all(|(_, val)| *val == 1.0));
    }

    #[test]
    fn test_get_id() {
        const ID: PlayerID = 1;
        let player = GuessingPlayer { id: ID };
        assert_eq!(player.get_id(), ID);
    }
}
