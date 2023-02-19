use crate::interfaces::*;

pub struct NaiveReferee<const N: usize, const K: usize> {}

fn evaluate_board<const N: usize, const K: usize>(
    board: &Board<N>,
    player: PlayerID,
) -> Option<Result> {
    let mut has_free_cells = false;
    let deltas = [
        (0, 1),  // horizontal
        (1, 0),  // vertical
        (1, 1),  // slash diagonal
        (1, -1), // backslash diagonal
    ];
    for row in 0..N {
        for column in 0..N {
            has_free_cells |= board.board[row][column].is_none();
            for cur in deltas {
                if has_winning_state_in_direction::<N, K>(cur, row, column, board, player) {
                    return Some(Result::Victory);
                }
            }
        }
    }
    if !has_free_cells {
        return Some(Result::Draw);
    }
    return None;
}

fn has_winning_state_in_direction<const N: usize, const K: usize>(
    delta: (i32, i32),
    start_row: usize,
    start_column: usize,
    board: &Board<N>,
    player: PlayerID,
) -> bool {
    let (dx, dy) = delta;
    let end_x: i32 = dx * (K - 1) as i32 + start_row as i32;
    let end_y: i32 = dy * (K - 1) as i32 + start_column as i32;
    if end_x < 0 || end_x >= N as i32 || end_y < 0 || end_y >= N as i32 {
        return false;
    }

    let mut has_won = true;
    for k in 0..K {
        let row = (start_row as i32 + dx * k as i32) as usize;
        let column = (start_column as i32 + dy * k as i32) as usize;
        has_won &= board.board[row as usize][column as usize] == Some(player);
    }

    return has_won;
}

impl<const N: usize, const K: usize> TicTacToeReferee<N, K> for NaiveReferee<N, K> {
    fn receive_move(
        &mut self,
        board: &mut Board<N>,
        placement: PointPlacement,
        player_id: PlayerID,
    ) -> Option<Result> {
        let (row, col) = (placement.row, placement.column);
        if board.has_placement_at(placement) {
            Some(Result::IllegalMove)
        } else {
            board.board[row][col] = Some(player_id);
            return evaluate_board::<N, K>(board, player_id);
        }
    }
}
