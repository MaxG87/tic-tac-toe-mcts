use crate::board::Board;
use std::collections::HashSet;
use std::fmt;

pub type BoardSizeT = u16;
pub type WinLengthT = u16;
pub type PlayerID = u16;
// TODO: Apply NewType idiom for Evaluation and Placement

pub type Evaluation = Board<f32>;
pub type Placement = Board<f32>;
pub type GameState = Board<BoardStateEntry>;

#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug)]
pub struct BoardStateEntry(Option<PlayerID>);

impl BoardStateEntry {
    pub fn is_taken(self) -> bool {
        self.0.is_some()
    }

    pub fn is_free(self) -> bool {
        !self.is_taken()
    }
}

impl fmt::Display for BoardStateEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(player_id) => write!(f, "{player_id}"),
            None => write!(f, "."),
        }
    }
}

impl From<Option<PlayerID>> for BoardStateEntry {
    fn from(value: Option<PlayerID>) -> Self {
        BoardStateEntry(value)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct PointPlacement {
    pub row: BoardSizeT,
    pub column: BoardSizeT,
}

impl PointPlacement {
    pub fn get_lower_neighbours(
        &self,
        nrows: BoardSizeT,
        ncolumns: BoardSizeT,
    ) -> impl Iterator<Item = PointPlacement> {
        let mut result = HashSet::new();
        let directions = [
            (0, -1),  // horizontal
            (-1, 0),  // vertical
            (-1, -1), // backslash diagonal
            (-1, 1),  // diagonal
        ];
        for (dx, dy) in directions {
            let next_row = i32::from(self.row) + dx;
            let next_column = i32::from(self.column) + dy;
            if next_row < 0
                || next_column < 0
                || next_row > i32::from(nrows)
                || next_column > i32::from(ncolumns)
            {
                continue;
            }

            let next_row = BoardSizeT::try_from(next_row)
                .expect("Value must be in bounds, as checked above!");
            let next_column = BoardSizeT::try_from(next_row)
                .expect("Value must be in bounds, as checked above!");

            let pp = PointPlacement {
                row: next_row,
                column: next_column,
            };
            result.insert(pp);
        }
        result.into_iter()
    }
}

impl fmt::Display for PointPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PointPlacement({}, {})", self.row, self.column)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum GameResult {
    Defeat,
    Draw,
    IllegalMove,
    Victory,
    Undecided,
}
impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::Defeat => write!(f, "Defeat"),
            GameResult::Draw => write!(f, "Draw"),
            GameResult::IllegalMove => write!(f, "IllegalMove"),
            GameResult::Undecided => write!(f, "Undecided"),
            GameResult::Victory => write!(f, "Victory"),
        }
    }
}

pub trait TicTacToeReferee {
    fn receive_move(
        &mut self,
        board: &mut GameState,
        placement: PointPlacement,
        player: PlayerID,
    ) -> GameResult;
}

pub trait Player {
    fn do_move(&mut self, board: &GameState) -> Placement;
    fn get_id(&self) -> PlayerID;
}

pub trait TicTacToeArena {
    fn do_next_move(&mut self) -> (GameResult, PlayerID, Option<PointPlacement>);
    fn get_board(&self) -> GameState;
}
