use crate::interfaces::{
    AbstractBoard, Board, BoardSizeT, PlayerID, PointPlacement, Result,
    TicTacToeReferee, WinLengthT,
};

pub struct NaiveReferee<const N: BoardSizeT, const K: WinLengthT> {}

fn evaluate_board<const N: BoardSizeT, const K: WinLengthT>(
    board: &dyn AbstractBoard<BoardSizeT>,
    player: PlayerID,
) -> Result {
    let mut has_free_cells = false;
    let deltas = [
        (0, 1),  // horizontal
        (1, 0),  // vertical
        (1, 1),  // slash diagonal
        (1, -1), // backslash diagonal
    ];

    for (pp, value) in board.flatten() {
        has_free_cells |= value.is_none();
        for cur in deltas {
            if has_winning_state_in_direction::<N, K>(
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

fn has_winning_state_in_direction<const N: BoardSizeT, const K: WinLengthT>(
    delta: (i32, i32),
    start_row: BoardSizeT,
    start_column: BoardSizeT,
    board: &dyn AbstractBoard<BoardSizeT>,
    player: PlayerID,
) -> bool {
    let (dx, dy) = delta;
    let end_x: i32 = dx * i32::from(K - 1) + start_row as i32;
    let end_y: i32 = dy * i32::from(K - 1) + start_column as i32;
    if end_x < 0 || end_x >= N as i32 || end_y < 0 || end_y >= N as i32 {
        return false;
    }

    let mut has_won = true;
    for k in 0..K {
        let row = (start_row as i32 + dx * i32::from(k)) as BoardSizeT;
        let column = (start_column as i32 + dy * i32::from(k)) as BoardSizeT;
        let pp = PointPlacement { row, column };
        has_won &= board.get_placement_at(pp) == Some(player);
    }

    has_won
}

impl<const N: BoardSizeT, const K: WinLengthT> TicTacToeReferee<N, K>
    for NaiveReferee<N, K>
{
    fn receive_move(
        &mut self,
        board: &mut Board<N>,
        placement: PointPlacement,
        player_id: PlayerID,
    ) -> Result {
        if board.has_placement_at(placement) {
            Result::IllegalMove
        } else {
            board.set_placement_at(placement, Some(player_id));
            evaluate_board::<N, K>(board, player_id)
        }
    }
}
