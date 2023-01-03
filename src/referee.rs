use crate::arena::*;

struct NaiveReferee<const N: u32> {
    board_state: BoardState,
}

fn was_winning_move<const N: u32>(
    placement: &Placement,
    board_state: &BoardState,
    player: &PlayerID,
) -> bool {
    return winning_state_in_row::<N>(placement.row, board_state, player)
        || winning_state_in_col::<N>(placement.col, board_state, player);
}

fn winning_state_in_row<const N: u32>(
    row: usize,
    board_state: &BoardState,
    player: &PlayerID,
) -> bool {
    let row_it = &board_state[row];
    return collection_has_winning_state::<N>(&mut row_it.iter(), &player);
}

fn winning_state_in_col<const N: u32>(
    col: usize,
    board_state: &BoardState,
    player: &PlayerID,
) -> bool {
    let mut column_it = board_state.iter().map(|row| &row[col]);
    return collection_has_winning_state::<N>(&mut column_it, &player);
}

fn collection_has_winning_state<const N: u32>(
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

impl<const N: u32> TicTacToeReferee<N> for NaiveReferee<N> {
    fn receive_move(&mut self, placement: &Placement, player_id: PlayerID) -> MoveResult {
        let (row, col) = (placement.row, placement.col);
        if let Some(_) = self.board_state[row][col] {
            MoveResult {
                state: &self.board_state,
                result: Some(Result::IllegalMove),
            }
        } else {
            self.board_state[row][col] = Some(player_id.clone());
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
