use crate::interfaces::{GameState, Placement, Player, PlayerID};

pub struct GuessingPlayer {
    pub id: PlayerID,
}

impl GuessingPlayer {}

impl Player for GuessingPlayer {
    fn do_move(&mut self, board: &GameState) -> Placement {
        Placement::new_from_existing(board, 1.0)
    }

    fn get_id(&self) -> PlayerID {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_guessing_player_uses_constant_probabilities() {
        const N: u16 = 10;
        const ID: PlayerID = 1;
        let mut player = GuessingPlayer { id: ID };
        let board = GameState::new(N, N, None);
        let placement = player.do_move(&board);
        let values = placement
            .into_iter_2d()
            .map(|(_, val)| val)
            .map(f32::to_bits)
            .collect::<HashSet<_>>();
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn test_get_id() {
        const ID: PlayerID = 1;
        let player = GuessingPlayer { id: ID };
        assert_eq!(player.id, ID);
    }
}
