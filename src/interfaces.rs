use crate::board::Board;
use std::fmt;

pub type BoardSizeT = usize;
pub type WinLengthT = u16;
pub type PlayerID = usize;
pub type BoardStateEntry = Option<PlayerID>;
// TODO: Apply NewType idiom for Evaluation and Placement
pub type Evaluation<const N: BoardSizeT> = [[f32; N]; N];
pub type Placement<const N: BoardSizeT> = [[f32; N]; N];

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct PointPlacement {
    pub row: BoardSizeT,
    pub column: BoardSizeT,
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

pub trait TicTacToeReferee<const K: WinLengthT> {
    fn receive_move(
        &mut self,
        board: &mut Board,
        placement: PointPlacement,
        player: PlayerID,
    ) -> Result;
}

pub trait Player<const N: usize, const K: WinLengthT> {
    fn do_move(&mut self, board: &Board) -> Placement<N>;
    fn get_id(&self) -> PlayerID;
}

pub trait TicTacToeArena<const N: usize, const K: WinLengthT> {
    fn do_next_move(&mut self) -> (Result, PlayerID, Option<PointPlacement>);
    fn get_board(&self) -> Board;
}
