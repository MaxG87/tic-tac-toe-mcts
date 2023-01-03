pub type Placement<const N: usize> = [[f32; N]; N];
pub type BoardStateEntry = Option<PlayerID>;

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
    pub fn get_row(&self, row: usize) -> &[BoardStateEntry; N] {
        &self.board[row]
    }
    pub fn get_column(&self, column: usize) -> Vec<&BoardStateEntry> {
        // TODO There must be a copy-free way to iterate over columns!
        self.board
            .iter()
            .map(|row| &row[column])
            .collect::<Vec<&BoardStateEntry>>()
    }
}

#[derive(PartialEq, Copy, Clone)]
pub struct PointPlacement {
    pub row: usize,
    pub col: usize,
}

pub enum Result {
    Defeat,
    Draw,
    IllegalMove,
    Victory,
}
pub struct MoveResult<'a, const N: usize> {
    pub state: &'a Board<N>,
    pub result: Option<Result>,
}
pub trait TicTacToeReferee<const N: usize> {
    fn receive_move(&mut self, placement: &PointPlacement, player: PlayerID) -> MoveResult<N>;
}

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerID {
    name: String,
    id: usize,
}

pub trait Player<const N: usize> {
    fn do_move(&mut self, board: &Board<N>) -> &Placement<N>;
}

pub struct TicTacToeArena<const N: usize> {
    player1: Box<dyn Player<N>>,
    player2: Box<dyn Player<N>>,
    referee: Box<dyn TicTacToeReferee<N>>,
}
