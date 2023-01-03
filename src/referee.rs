use crate::arena::*;

pub struct NaiveReferee<const N: usize, const K: usize> {}

fn was_winning_move<const N: usize, const K: usize>(
    placement: &PointPlacement,
    board_state: &Board<N>,
    player: &PlayerID,
) -> bool {
    return winning_state_in_row::<N, K>(placement.row, board_state, player)
        || winning_state_in_col::<N, K>(placement.column, board_state, player);
}

fn winning_state_in_row<const N: usize, const K: usize>(
    row: usize,
    board_state: &Board<N>,
    player: &PlayerID,
) -> bool {
    let row = board_state.get_row(row);
    return collection_has_winning_state::<N, K>(&mut row.iter(), &player);
}

fn winning_state_in_col<const N: usize, const K: usize>(
    column: usize,
    board_state: &Board<N>,
    player: &PlayerID,
) -> bool {
    let column = board_state.get_column(column);
    return collection_has_winning_state::<N, K>(&mut column.into_iter(), player);
}

fn collection_has_winning_state<const N: usize, const K: usize>(
    collection: &mut dyn Iterator<Item = &BoardStateEntry>,
    player: &PlayerID,
) -> bool {
    let mut counter = 0;
    for elem in collection {
        match elem {
            None => counter = 0,
            Some(p) => {
                if p == player {
                    counter += 1
                } else {
                    counter = 0
                }
            }
        }
        if counter == K {
            return true;
        }
    }
    return false;
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
            let was_winning_move = was_winning_move::<N, K>(placement, board, &player_id);
            if was_winning_move {
                Some(Result::Victory)
            } else {
                None
            }
        }
    }
}
