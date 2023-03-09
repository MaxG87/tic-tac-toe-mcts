use std::fmt;

pub type BoardStateEntry = Option<PlayerID>;
pub type Evaluation<const N: usize> = [[f32; N]; N];
pub type Placement<const N: usize> = [[f32; N]; N];
pub type PlayerID = usize;

#[derive(Clone)]
pub struct Board<const N: usize> {
    pub board: [[BoardStateEntry; N]; N],
}

impl<const N: usize> Board<N> {
    pub fn rows(&self) -> usize {
        N
    }
    pub fn columns(&self) -> usize {
        N
    }
    pub fn has_placement_at(&self, pp: PointPlacement) -> bool {
        self.board[pp.row][pp.column].is_some()
    }
}

impl<const N: usize> fmt::Display for Board<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows() {
            for column in 0..(self.columns() - 1) {
                write!(
                    f,
                    "{}",
                    match self.board[row][column] {
                        Some(id) => format!("{id}"),
                        None => ".".to_string(),
                    }
                )?
            }
            writeln!(
                f,
                "{}",
                match self.board[row][self.columns() - 1] {
                    Some(id) => format!("{id}"),
                    None => ".".to_string(),
                }
            )?
        }
        fmt::Result::Ok(())
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct PointPlacement {
    pub row: usize,
    pub column: usize,
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

pub trait TicTacToeReferee<const N: usize, const K: usize> {
    fn receive_move(
        &mut self,
        board: &mut Board<N>,
        placement: PointPlacement,
        player: PlayerID,
    ) -> Result;
}

pub trait Player<const N: usize, const K: usize> {
    fn do_move(&mut self, board: &Board<N>) -> Placement<N>;
    fn get_id(&self) -> PlayerID;
}

pub trait TicTacToeArena<const N: usize, const K: usize> {
    fn do_next_move(&mut self) -> (Result, PlayerID, Option<PointPlacement>);
    fn get_board(&self) -> Board<N>;
}
