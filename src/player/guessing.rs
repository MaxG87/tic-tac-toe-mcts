use crate::board::Board;
use crate::interfaces::{BoardSizeT, Placement, Player, PlayerID, WinLengthT};

pub struct GuessingPlayer<const N: BoardSizeT, const K: WinLengthT> {
    pub id: PlayerID,
}

impl<const N: BoardSizeT, const K: WinLengthT> GuessingPlayer<N, K> {
    const PLACEMENT: Placement<N> = [[1.0; N]; N];
}

impl<const N: BoardSizeT, const K: WinLengthT> Player<N, K> for GuessingPlayer<N, K> {
    fn do_move(&mut self, _: &Board) -> Placement<N> {
        GuessingPlayer::<N, K>::PLACEMENT
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
        const N: BoardSizeT = 10;
        const K: WinLengthT = 3;
        const ID: PlayerID = 1;
        let mut player = GuessingPlayer::<N, K> { id: ID };
        let board = Board::new(u16::try_from(N).unwrap(), u16::try_from(N).unwrap());
        let placement = player.do_move(&board);
        let values = placement
            .into_iter()
            .flat_map(std::iter::IntoIterator::into_iter)
            .map(f32::to_bits)
            .collect::<HashSet<_>>();
        assert_eq!(values.len(), 1);
    }

    #[test]
    fn test_get_id() {
        const N: BoardSizeT = 10;
        const K: WinLengthT = 3;
        const ID: PlayerID = 1;
        let player = GuessingPlayer::<N, K> { id: ID };
        assert_eq!(player.id, ID);
    }
}
