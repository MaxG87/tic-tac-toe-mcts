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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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
            let new_row = self.row.saturating_add
            let new_column = self.column as i32 + dy;
            if new_row >= 0 && new_row < nrows as i32 && new_column >= 0
                && new_column < ncolumns as i32
            {
                result.insert(PointPlacement {
                    row: new_row as BoardSizeT,
                    column: new_column as BoardSizeT,
                });
            }

        }
        result.into_iter()
    }
}

impl fmt::Display for PointPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PointPlacement({}, {})", self.row, self.column)
    }
}

#[derive(PartialEq, Eq)]
pub enum Result {
    Defeat,
    Draw,
    IllegalMove,
    Victory,
    Undecided,
}
impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Result::Defeat => write!(f, "Defeat"),
            Result::Draw => write!(f, "Draw"),
            Result::IllegalMove => write!(f, "IllegalMove"),
            Result::Undecided => write!(f, "Undecided"),
            Result::Victory => write!(f, "Victory"),
        }
    }
}

pub trait TicTacToeReferee {
    fn receive_move(
        &mut self,
        board: &mut GameState,
        placement: PointPlacement,
        player: PlayerID,
    ) -> Result;
}

pub trait Player {
    fn do_move(&mut self, board: &GameState) -> Placement;
    fn get_id(&self) -> PlayerID;
}

pub trait TicTacToeArena {
    fn do_next_move(&mut self) -> (Result, PlayerID, Option<PointPlacement>);
    fn get_board(&self) -> GameState;
}
