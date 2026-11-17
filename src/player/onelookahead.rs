use crate::interfaces::{
    GameResult, GameState, Placement, Player, PlayerID, TicTacToeReferee,
};

pub struct OneLookaheadPlayer {
    other_id: PlayerID,
    referee: Box<dyn TicTacToeReferee>,
    self_id: PlayerID,
}

impl OneLookaheadPlayer {
    #[allow(dead_code)]
    pub fn new(
        other_id: PlayerID,
        referee: Box<dyn TicTacToeReferee>,
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
            0.0_f32,
        );
        let mut mut_board = board.clone();
        for (pp, old_val) in board.iter_2d() {
            if GameResult::Victory
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
            0.0_f32,
        );
        for (pp, old_val) in board.iter_2d() {
            if GameResult::Victory
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

impl Player for OneLookaheadPlayer {
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
            1.0_f32,
        )
    }

    fn get_id(&self) -> PlayerID {
        self.self_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::referee::NaiveReferee;
    use crate::test_utils::{board, placement};
    use rstest::rstest;

    #[rstest]
    // takes an immediate win
    #[case(
        vec![vec![Some(0), Some(0), None], vec![Some(1), None, None], vec![None, None, None]],
        3,
        vec![vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]],
    )]
    // all winning moves are returned
    #[case(
        vec![vec![Some(0), Some(0), None], vec![Some(0), Some(1), None], vec![None, Some(1), None]],
        3,
        vec![vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 0.0], vec![1.0, 0.0, 0.0]],
    )]
    // blocks a horizontal threat
    #[case(
        vec![vec![Some(1), Some(1), None], vec![Some(0), None, None], vec![None, None, None]],
        3,
        vec![vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]],
    )]
    // blocks a vertical threat
    #[case(
        vec![vec![Some(1), Some(0), None], vec![Some(1), None, None], vec![None, None, None]],
        3,
        vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0], vec![1.0, 0.0, 0.0]],
    )]
    // blocks a diagonal threat
    #[case(
        vec![vec![Some(1), None, None], vec![Some(0), Some(1), None], vec![None, None, None]],
        3,
        vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 1.0]],
    )]
    // winning takes precedence over blocking
    #[case(
        vec![vec![Some(0), Some(0), None], vec![Some(1), Some(1), None], vec![None, None, None]],
        3,
        vec![vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]],
    )]
    // no tactic: all cells are equally weighted
    #[case(
        vec![vec![None, None, None], vec![None, None, None], vec![None, None, None]],
        3,
        vec![vec![1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0]],
    )]
    fn finds_immediate_tactics(
        #[case] initial_board: Vec<Vec<Option<PlayerID>>>,
        #[case] win_length: u16,
        #[case] expected: Vec<Vec<f32>>,
    ) {
        let mut player =
            OneLookaheadPlayer::new(1, Box::new(NaiveReferee::new(win_length)), 0);

        let placements = player.do_move(&board(initial_board));

        assert_eq!(placements, placement(expected));
    }
}
