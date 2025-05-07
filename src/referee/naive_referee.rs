use crate::interfaces::{
    BoardSizeT, GameResult, GameState, PlayerID, PointPlacement, TicTacToeReferee,
    WinLengthT,
};
use anyhow::Context;

const DELTAS: [Direction; 4] = [
    Direction {
        // horizontal
        row_delta: 0,
        column_delta: 1,
    },
    Direction {
        // vertical
        row_delta: 1,
        column_delta: 0,
    },
    Direction {
        // slash diagonal
        row_delta: 1,
        column_delta: 1,
    },
    Direction {
        // backslash diagonal
        row_delta: 1,
        column_delta: -1,
    },
];

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NaiveReferee {
    winning_length: WinLengthT,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Direction {
    row_delta: i32,
    column_delta: i32,
}

impl Direction {
    fn try_add(&self, other: PointPlacement) -> anyhow::Result<PointPlacement> {
        let row = i32::from(other.row) + self.row_delta;
        let row = BoardSizeT::try_from(row).context("Row out of bounds")?;
        let column = i32::from(other.column) + self.column_delta;
        let column = BoardSizeT::try_from(column).context("Column out of bounds")?;
        Ok(PointPlacement { row, column })
    }
}

impl NaiveReferee {
    #[must_use]
    pub fn new(winning_length: u16) -> Self {
        NaiveReferee { winning_length }
    }

    #[inline]
    fn follow_direction(
        &self,
        start_pp: PointPlacement,
        direction: &Direction,
        max_row: BoardSizeT,
        max_column: BoardSizeT,
    ) -> impl Iterator<Item = PointPlacement> {
        let pp_in_direction =
            (1..self.winning_length).scan(start_pp, move |cur_pp, _| {
                let maybe_pp = direction.try_add(*cur_pp);
                match maybe_pp {
                    Ok(new_pp)
                        if new_pp.row < max_row && new_pp.column < max_column =>
                    {
                        *cur_pp = new_pp;
                        Some(new_pp)
                    }
                    _ => None,
                }
            });
        std::iter::once(start_pp).chain(pp_in_direction)
    }

    fn evaluate_board(&self, board: &GameState, player: PlayerID) -> GameResult {
        let mut has_free_cells = false;

        for (pp, value) in board.iter_2d() {
            has_free_cells |= value.is_free();
            for cur in &DELTAS {
                if self.has_winning_state_in_direction(cur, pp, board, player) {
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
        direction: &Direction,
        start_pp: PointPlacement,
        board: &GameState,
        player: PlayerID,
    ) -> bool {
        let relevant_placements = self.follow_direction(
            start_pp,
            direction,
            board.get_number_of_rows(),
            board.get_number_of_columns(),
        );
        let mut consecutive_placements = 0;
        for pp in relevant_placements {
            consecutive_placements += 1;
            if board[pp] != Some(player).into() {
                return false;
            }
        }
        // relevant_placements might contain less than self.winning_length placements.
        // Since it is a lazy iterator, this cannot be checked in advance.
        consecutive_placements == self.winning_length
    }
}

impl TicTacToeReferee for NaiveReferee {
    fn receive_move(
        &self,
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
        let referee = NaiveReferee::new(winning_length);
        let result = referee.receive_move(&mut board, next_move, player);
        assert_eq!(result, expected);
    }
}
