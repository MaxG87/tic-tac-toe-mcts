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
        let (dx, dy) = delta;
        let end_x: i32 = dx * i32::from(self.winning_length - 1) + i32::from(start_row);
        let end_y: i32 =
            dy * i32::from(self.winning_length - 1) + i32::from(start_column);
        if end_x < 0
            || end_x >= i32::from(ncolumns)
            || end_y < 0
            || end_y >= i32::from(nrows)
        {
            return false;
        }

        let mut has_won = true;
        for k in 0..self.winning_length {
            let row = (i32::from(start_row) + dx * i32::from(k)) as BoardSizeT;
            let column = (i32::from(start_column) + dy * i32::from(k)) as BoardSizeT;
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
        if board[placement].is_taken() {
            // There is already a player on this cell.
            GameResult::IllegalMove
        } else {
            board[placement] = Some(player_id).into();
            self.evaluate_board(board, player_id)
        }
    }
}
