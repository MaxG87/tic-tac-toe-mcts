use crate::interfaces::{
    BoardSizeT, GameState, PlayerID, PointPlacement, Result, TicTacToeReferee,
    WinLengthT,
};

pub struct NaiveReferee<const K: WinLengthT> {}

fn evaluate_board<const K: WinLengthT>(board: &GameState, player: PlayerID) -> Result {
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
            if has_winning_state_in_direction::<K>(
                cur, pp.row, pp.column, board, player,
            ) {
                return Result::Victory;
            }
        }
    }
    if !has_free_cells {
        return Result::Draw;
    }
    Result::Undecided
}

fn has_winning_state_in_direction<const K: WinLengthT>(
    delta: (i32, i32),
    start_row: BoardSizeT,
    start_column: BoardSizeT,
    board: &GameState,
    player: PlayerID,
) -> bool {
    let nrows = board.get_number_of_rows();
    let ncolumns = board.get_number_of_columns();
    let (dx, dy) = delta;
    let end_x: i32 = dx * i32::from(K - 1) + start_row as i32;
    let end_y: i32 = dy * i32::from(K - 1) + start_column as i32;
    if end_x < 0 || end_x >= ncolumns as i32 || end_y < 0 || end_y >= nrows as i32 {
        return false;
    }

    let mut has_won = true;
    for k in 0..K {
        let row = (start_row as i32 + dx * i32::from(k)) as BoardSizeT;
        let column = (start_column as i32 + dy * i32::from(k)) as BoardSizeT;
        let pp = PointPlacement { row, column };
        has_won &= board[pp] == Some(player).into();
    }

    has_won
}

impl<const K: WinLengthT> TicTacToeReferee<K> for NaiveReferee<K> {
    fn receive_move(
        &mut self,
        board: &mut GameState,
        placement: PointPlacement,
        player_id: PlayerID,
    ) -> Result {
        if board[placement].is_taken() {
            // There is already a player on this cell.
            Result::IllegalMove
        } else {
            board[placement] = Some(player_id).into();
            evaluate_board::<K>(board, player_id)
        }
    }
}
