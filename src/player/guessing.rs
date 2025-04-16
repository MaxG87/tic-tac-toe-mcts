use crate::interfaces::{GameState, Placement, Player, PlayerID, WinLengthT};

pub struct GuessingPlayer<const K: WinLengthT> {
    pub id: PlayerID,
}

impl<const K: WinLengthT> GuessingPlayer<K> {}

impl<const K: WinLengthT> Player<K> for GuessingPlayer<K> {
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
        const K: WinLengthT = 3;
        const ID: PlayerID = 1;
        let mut player = GuessingPlayer::<K> { id: ID };
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
        const K: WinLengthT = 3;
        const ID: PlayerID = 1;
        let player = GuessingPlayer::<K> { id: ID };
        assert_eq!(player.id, ID);
    }
}
