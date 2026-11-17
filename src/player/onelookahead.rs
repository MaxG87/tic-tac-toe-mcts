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

    fn player() -> OneLookaheadPlayer {
        OneLookaheadPlayer::new(1, Box::new(NaiveReferee::new(3)), 0)
    }

    #[test]
    fn prefers_an_immediate_winning_move() {
        let board = GameState::new_with_values([
            [Some(0), Some(0), None],
            [Some(1), None, None],
            [None, None, None],
        ])
        .unwrap();
        let mut player = player();

        let placements = player.do_move(&board);

        for (pp, weight) in placements.iter_2d() {
            let expected =
                if pp == (crate::interfaces::PointPlacement { row: 0, column: 2 }) {
                    1.0
                } else {
                    0.0
                };
            assert_eq!(*weight, expected, "unexpected weight at {pp}");
        }
    }

    #[test]
    fn blocks_an_opponents_immediate_win_when_no_win_is_available() {
        let board = GameState::new_with_values([
            [Some(1), Some(1), None],
            [Some(0), None, None],
            [None, None, None],
        ])
        .unwrap();
        let mut player = player();

        let placements = player.do_move(&board);

        for (pp, weight) in placements.iter_2d() {
            let expected =
                if pp == (crate::interfaces::PointPlacement { row: 0, column: 2 }) {
                    1.0
                } else {
                    0.0
                };
            assert_eq!(*weight, expected, "unexpected weight at {pp}");
        }
    }

    #[test]
    fn returns_uniform_weights_when_no_immediate_tactic_exists() {
        let board = GameState::new(3, 3, None);
        let mut player = player();

        let placements = player.do_move(&board);

        assert!(placements.iter_2d().all(|(_, weight)| *weight == 1.0));
    }
}
