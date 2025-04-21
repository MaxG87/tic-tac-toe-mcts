use crate::interfaces::{
    BoardSizeT, GameResult, GameState, PlayerID, PointPlacement, TicTacToeReferee,
    WinLengthT,
};

pub struct NaiveReferee {
    winning_length: WinLengthT,
}

impl NaiveReferee {
    pub fn new(winning_length: u16) -> Self {
        NaiveReferee { winning_length }
    }

    fn evaluate_board(&self, board: &GameState, player: PlayerID) -> GameResult {
        let mut has_free_cells = false;
        let deltas = [
            (0, 1),  // horizontal
            (1, 0),  // vertical
            (1, 1),  // slash diagonal
            (1, -1), // backslash diagonal
        ];

        for (pp, value) in board.iter_2d() {
            has_free_cells |= value.is_free();
            for cur in deltas {
                if self.has_winning_state_in_direction(
                    cur, pp.row, pp.column, board, player,
                ) {
                    return GameResult::Victory;
                }
            }
        }
        if !has_free_cells {
            return GameResult::Draw;
        }
        GameResult::Undecided
    }

    fn has_winning_state_in_direction(
        &self,
        delta: (i32, i32),
        start_row: BoardSizeT,
        start_column: BoardSizeT,
        board: &GameState,
        player: PlayerID,
    ) -> bool {
        let nrows = board.get_number_of_rows();
        let ncolumns = board.get_number_of_columns();
        let (drow, dcolumn) = delta;
        let end_row: i32 =
            drow * i32::from(self.winning_length - 1) + i32::from(start_row);
        let end_column: i32 =
            dcolumn * i32::from(self.winning_length - 1) + i32::from(start_column);
        if end_row < 0
            || end_row >= i32::from(nrows)
            || end_column < 0
            || end_column >= i32::from(ncolumns)
        {
            return false;
        }

        let mut has_won = true;
        for k in 0..self.winning_length {
            let row = (i32::from(start_row) + drow * i32::from(k)) as BoardSizeT;
            let column =
                (i32::from(start_column) + dcolumn * i32::from(k)) as BoardSizeT;
            let pp = PointPlacement { row, column };
            has_won &= board[pp] == Some(player).into();
        }

        has_won
    }
}

impl TicTacToeReferee for NaiveReferee {
    fn receive_move(
        &mut self,
        board: &mut GameState,
        placement: PointPlacement,
        player_id: PlayerID,
    ) -> GameResult {
        if placement.row >= board.get_number_of_rows()
            || placement.column >= board.get_number_of_columns()
        {
            // Placement is out of bounds.
            return GameResult::IllegalMove;
        }
        if board[placement].is_taken() {
            // There is already a player on this cell.
            return GameResult::IllegalMove;
        }
        board[placement] = Some(player_id).into();
        self.evaluate_board(board, player_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    // horizontal
    #[case(GameState::new_with_values(
        [
            [None, Some(0), Some(0), None],
            [None, Some(1), Some(1), None],
            [None, None, None, None],
        ]
    ).unwrap(),
        PointPlacement{row: 0, column: 0},
        3,
        0,
        GameResult::Victory
    )]
    // vertical
    #[case(GameState::new_with_values(
        [
            [None, Some(0), Some(1)],
            [None, Some(0), Some(1)],
            [None, None, None],
        ]
    ).unwrap(),
        PointPlacement{row: 2, column: 2},
        3,
        1,
        GameResult::Victory
    )]
    // slash diagonal
    #[case(GameState::new_with_values(
        [
            [None, None, Some(1), Some(0)],
            [None, None, Some(0), Some(1)],
            [None, Some(0), Some(1), None],
            [None, None, None, None],
        ]
    ).unwrap(),
        PointPlacement{row: 3, column: 0},
        4,
        0,
        GameResult::Victory
    )]
    // backslash diagonal
    #[case(GameState::new_with_values(
        [
            [None,    None,    None,    None,    None,    None],
            [None,    None,    None,    None,    None,    None],
            [Some(0), None,    None,    None,    None,    None],
            [Some(1), Some(0), None,    None,    None,    None],
            [None,    Some(1), Some(0), None,    None,    None],
            [None,    None,    Some(1), Some(0), None,    None],
            [None,    None,    None,    Some(1), Some(0), None],
            [None,    None,    None,    None,    Some(1), None],
        ]
    ).unwrap(),
        PointPlacement{row: 7, column: 5},
        6,
        0,
        GameResult::Victory,
    )]
    // Illegal move - cell already taken
    #[case(GameState::new_with_values(
        [
            [Some(0), None],
            [None, None]
        ]
    ).unwrap(),
        PointPlacement{row: 0, column: 0},
        2,
        1,
        GameResult::IllegalMove,
    )]
    // Illegal move - placement out of bounds
    #[case(GameState::new_with_values(
        [
            [Some(0), Some(1)],
            [None, None]
        ]
    ).unwrap(),
        PointPlacement{row: 2, column: 0},
        2,
        0,
        GameResult::IllegalMove,
    )]
    // undecided
    #[case(GameState::new_with_values(
        [
            [None, None, Some(1), Some(0)],
            [None, None, Some(0), Some(1)],
            [None, Some(0), Some(1), None],
            [None, None, None, None],
        ]
    ).unwrap(),
        PointPlacement{row: 2, column: 0},
        4,
        1,
        GameResult::Undecided,
    )]
    fn referee_judges_board_correctly(
        #[case] mut board: GameState,
        #[case] next_move: PointPlacement,
        #[case] winning_length: WinLengthT,
        #[case] player: PlayerID,
        #[case] expected: GameResult,
    ) {
        let mut referee = NaiveReferee::new(winning_length);
        let result = referee.receive_move(&mut board, next_move, player);
        assert_eq!(result, expected);
    }
}
