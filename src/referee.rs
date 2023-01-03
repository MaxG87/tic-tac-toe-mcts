use crate::arena::*;

pub struct NaiveReferee<const N: usize, const K: usize> {}

fn evaluate_board<const N: usize, const K: usize>(
    board: &Board<N>,
    player: &PlayerID,
) -> Option<Result> {
    let mut nof_free_cells = 0;
    for row in 0..N {
        for column in 0..N {
            nof_free_cells += if let None = board.board[row][column] {
                1
            } else {
                0
            };
            if winning_state_in_row::<N, K>(board, row, column, player)
                || winning_state_in_column::<N, K>(board, row, column, player)
            {
                return Some(Result::Victory);
            }
        }
    }
    if nof_free_cells == 0 {
        return Some(Result::Draw);
    }
    return None;
}

fn winning_state_in_row<const N: usize, const K: usize>(
    board: &Board<N>,
    row: usize,
    column: usize,
    player: &PlayerID,
) -> bool {
    if column + 1 < K {
        return false;
    }
    for c in column + 1 - K..column + 1 {
        if let None = board.board[row][c] {
            return false;
        }
        if let Some(other) = board.board[row][c] {
            if other != *player {
                return false;
            }
        }
    }
    return true;
}

fn winning_state_in_column<const N: usize, const K: usize>(
    board: &Board<N>,
    row: usize,
    column: usize,
    player: &PlayerID,
) -> bool {
    if row + 1 < K {
        return false;
    }
    for r in row + 1 - K..row + 1 {
        if let None = board.board[r][column] {
            return false;
        }
        if let Some(other) = board.board[r][column] {
            if other != *player {
                return false;
            }
        }
    }
    return true;
}

impl<const N: usize, const K: usize> TicTacToeReferee<N, K> for NaiveReferee<N, K> {
    fn receive_move(
        &mut self,
        board: &mut Board<N>,
        placement: &PointPlacement,
        player_id: PlayerID,
    ) -> Option<Result> {
        let (row, col) = (placement.row, placement.column);
        if let Some(_) = board.board[row][col] {
            Some(Result::IllegalMove)
        } else {
            board.board[row][col] = Some(player_id);
            return evaluate_board::<N, K>(board, &player_id);
        }
    }
}
