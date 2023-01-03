use crate::arena::*;

struct NaiveReferee<const N: usize> {
    board_state: Board<N>,
}

fn was_winning_move<const N: usize>(
    placement: &PointPlacement,
    board_state: &Board<N>,
    player: &PlayerID,
) -> bool {
    return winning_state_in_row::<N>(placement.row, board_state, player)
        || winning_state_in_col::<N>(placement.col, board_state, player);
}

fn winning_state_in_row<const N: usize>(
    row: usize,
    board_state: &Board<N>,
    player: &PlayerID,
) -> bool {
    let row = board_state.get_row(row);
    return collection_has_winning_state::<N>(&mut row.iter(), &player);
}

fn winning_state_in_col<const N: usize>(
    column: usize,
    board_state: &Board<N>,
    player: &PlayerID,
) -> bool {
    let column = board_state.get_column(column);
    return collection_has_winning_state::<N>(&mut column.into_iter(), player);
}

fn collection_has_winning_state<const N: usize>(
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
        if counter == N {
            return true;
        }
    }
    return false;
}

impl<const N: usize> TicTacToeReferee<N> for NaiveReferee<N> {
    fn receive_move(&mut self, placement: &PointPlacement, player_id: PlayerID) -> MoveResult<N> {
        let (row, col) = (placement.row, placement.col);
        if let Some(_) = self.board_state.board[row][col] {
            MoveResult {
                state: &self.board_state,
                result: Some(Result::IllegalMove),
            }
        } else {
            self.board_state.board[row][col] = Some(player_id.clone());
            let was_winning_move = was_winning_move::<N>(placement, &self.board_state, &player_id);
            let result: Option<Result> = if was_winning_move {
                Some(Result::Victory)
            } else {
                None
            };
            MoveResult {
                state: &self.board_state,
                result: result,
            }
        }
    }
}
