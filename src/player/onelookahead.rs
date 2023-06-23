use crate::interfaces::*;
use crate::utils::*;

pub struct OneLookaheadPlayer<const N: usize, const K: usize> {
    other_id: PlayerID,
    referee: Box<dyn TicTacToeReferee<N, K>>,
    self_id: PlayerID,
}

impl<const N: usize, const K: usize> OneLookaheadPlayer<N, K> {
    const DEFAULT_PLACEMENT: Placement<N> = [[1.0; N]; N];

    pub fn new(
        other_id: PlayerID,
        referee: Box<dyn TicTacToeReferee<N, K>>,
        self_id: PlayerID,
    ) -> OneLookaheadPlayer<N, K> {
        OneLookaheadPlayer {
            other_id,
            referee,
            self_id,
        }
    }

    fn get_loosing_moves(&mut self, board: &Board<N>, placements: &mut Placement<N>) -> bool {
        let mut has_loosing_move = false;
        for (row, column, placement_cell) in iter_mut_2d_array(placements) {
            let pp = PointPlacement { row, column };
            let mut mut_board = board.clone();
            if Result::Victory == self.referee.receive_move(&mut mut_board, pp, self.other_id) {
                *placement_cell = 1.0;
                has_loosing_move = true;
            }
        }

        has_loosing_move
    }

    fn get_winning_moves(&mut self, board: &Board<N>, placements: &mut Placement<N>) -> bool {
        let mut has_winning_move = false;
        for (row, column, placement_cell) in iter_mut_2d_array(placements) {
            let pp = PointPlacement { row, column };
            let mut mut_board = board.clone();
            if Result::Victory == self.referee.receive_move(&mut mut_board, pp, self.get_id()) {
                *placement_cell = 1.0;
                has_winning_move = true;
            }
        }

        has_winning_move
    }
}

impl<const N: usize, const K: usize> Player<N, K> for OneLookaheadPlayer<N, K> {
    fn do_move(&mut self, board: &Board<N>) -> Placement<N> {
        let mut placements: Placement<N> = [[0.0; N]; N];
        if self.get_winning_moves(board, &mut placements) {
            return placements;
        }
        if self.get_loosing_moves(board, &mut placements) {
            return placements;
        }
        OneLookaheadPlayer::<N, K>::DEFAULT_PLACEMENT
    }

    fn get_id(&self) -> PlayerID {
        self.self_id
    }
}
