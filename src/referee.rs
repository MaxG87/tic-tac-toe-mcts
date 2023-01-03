type BoardStateEntry = Option<Player>;
type BoardState = Vec<Vec<BoardStateEntry>>;

enum Result {
    VICTORY,
    DRAW,
    DEFEAT,
}

#[derive(PartialEq)]
struct Player {
    name: String,
    id: u32,
}

struct MoveResult<'a> {
    state: &'a BoardState,
    result: Option<Vec<Result>>,
}

struct TicTacToeJudge<const N: u32> {
    board_state: BoardState,
}

impl<const N: u32> TicTacToeJudge<N> {
    pub fn receive_move(&mut self, row: usize, col: usize, player: Player) -> MoveResult {
        match self.board_state[row][col] {
            None => {
                self.board_state[row][col] = Some(player);
                MoveResult {
                    state: &self.board_state,
                    result: None,
                }
            }
            Some(_) => MoveResult {
                state: &self.board_state,
                result: None,
            },
        }
    }

    pub fn was_winning_move(&self, row: usize, col: usize, player: &Player) -> bool {
        return self.winning_state_in_row(row, player) || self.winning_state_in_col(col, player);
    }

    pub fn winning_state_in_row(&self, row: usize, player: &Player) -> bool {
        let row_it = &self.board_state[row];
        return self.collection_has_winning_state(&mut row_it.iter(), &player);
    }

    pub fn winning_state_in_col(&self, col: usize, player: &Player) -> bool {
        let mut column_it = self.board_state.iter().map(|row| &row[col]);
        return self.collection_has_winning_state(&mut column_it, &player);
    }

    pub fn collection_has_winning_state(
        &self,
        collection: &mut dyn Iterator<Item = &BoardStateEntry>,
        player: &Player,
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
}
