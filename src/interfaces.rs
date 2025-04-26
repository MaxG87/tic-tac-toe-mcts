use crate::board::Board;
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
        &self,
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
