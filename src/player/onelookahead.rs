use crate::interfaces::{
    GameState, Placement, Player, PlayerID, Result, TicTacToeReferee, WinLengthT,
};

pub struct OneLookaheadPlayer<const K: WinLengthT> {
    other_id: PlayerID,
    referee: Box<dyn TicTacToeReferee<K>>,
    self_id: PlayerID,
}

impl<const K: WinLengthT> OneLookaheadPlayer<K> {
    #[allow(dead_code)]
    pub fn new(
        other_id: PlayerID,
        referee: Box<dyn TicTacToeReferee<K>>,
        self_id: PlayerID,
    ) -> Self {
        Self {
            other_id,
            referee,
            self_id,
        }
    }

    fn get_loosing_moves(&mut self, board: &GameState) -> (bool, Placement) {
        let mut has_loosing_move = false;
        let mut placements = Placement::new(
            board.get_number_of_rows(),
            board.get_number_of_columns(),
            1.0,
        );
        let mut mut_board = board.clone();
        for (pp, old_val) in board.iter_2d() {
            if Result::Victory
                == self.referee.receive_move(&mut mut_board, pp, self.other_id)
            {
                placements[pp] = 1.0;
                has_loosing_move = true;
            }
            mut_board[pp] = *old_val;
        }

        (has_loosing_move, placements)
    }

    fn get_winning_moves(&mut self, board: &GameState) -> (bool, Placement) {
        let mut has_winning_move = false;
        let mut mut_board = board.clone();
        let mut placements = Placement::new(
            board.get_number_of_rows(),
            board.get_number_of_columns(),
            0.0,
        );
        for (pp, old_val) in board.iter_2d() {
            if Result::Victory
                == self.referee.receive_move(&mut mut_board, pp, self.get_id())
            {
                placements[pp] = 1.0;
                has_winning_move = true;
            }
            mut_board[pp] = *old_val;
        }

        (has_winning_move, placements)
    }
}

impl<const K: WinLengthT> Player<K> for OneLookaheadPlayer<K> {
    fn do_move(&mut self, board: &GameState) -> Placement {
        let (has_winning_move, placements) = self.get_winning_moves(board);
        if has_winning_move {
            return placements;
        }

        let (has_loosing_move, placements) = self.get_loosing_moves(board);
        if has_loosing_move {
            return placements;
        }

        Placement::new(
            board.get_number_of_rows(),
            board.get_number_of_columns(),
            1.0,
        )
    }

    fn get_id(&self) -> PlayerID {
        self.self_id
    }
}
