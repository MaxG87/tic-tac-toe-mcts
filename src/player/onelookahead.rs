use crate::interfaces::*;

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
        return OneLookaheadPlayer {
            other_id,
            referee,
            self_id,
        };
    }

    fn get_loosing_moves(&mut self, board: &Board<N>, placements: &mut Placement<N>) -> bool {
        let mut has_loosing_move = false;
        for row in 0..board.rows() {
            for column in 0..board.columns() {
                let pp = PointPlacement { row, column };
                let mut mut_board = board.clone();
                match self
                    .referee
                    .receive_move(&mut mut_board, &pp, self.other_id)
                {
                    Some(Result::Victory) => {
                        placements[row][column] = 1.0;
                        has_loosing_move = true;
                    }
                    _ => {}
                }
            }
        }

        return has_loosing_move;
    }

    fn get_winning_moves(&mut self, board: &Board<N>, placements: &mut Placement<N>) -> bool {
        let mut has_winning_move = false;
        for row in 0..board.rows() {
            for column in 0..board.columns() {
                let pp = PointPlacement { row, column };
                let mut mut_board = board.clone();
                match self
                    .referee
                    .receive_move(&mut mut_board, &pp, self.get_id())
                {
                    Some(Result::Victory) => {
                        placements[row][column] = 1.0;
                        has_winning_move = true;
                    }
                    _ => {}
                }
            }
        }

        return has_winning_move;
    }
}

impl<const N: usize, const K: usize> Player<N, K> for OneLookaheadPlayer<N, K> {
    fn do_move(&mut self, board: &Board<N>) -> Placement<N> {
        let mut placements: Placement<N> = [[0.0; N]; N];
        if self.get_winning_moves(&board, &mut placements) {
            return placements;
        }
        if self.get_loosing_moves(&board, &mut placements) {
            return placements;
        }
        return OneLookaheadPlayer::<N, K>::DEFAULT_PLACEMENT.clone();
    }

    fn get_id(&self) -> PlayerID {
        return self.self_id;
    }
}
