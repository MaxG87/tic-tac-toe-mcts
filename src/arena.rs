pub type BoardStateEntry = Option<PlayerID>;
pub type BoardState = Vec<Vec<BoardStateEntry>>;

#[derive(PartialEq, Copy, Clone)]
pub struct Placement {
    pub row: usize,
    pub col: usize,
}

pub enum Result {
    Defeat,
    Draw,
    IllegalMove,
    Victory,
}
pub struct MoveResult<'a> {
    pub state: &'a BoardState,
    pub result: Option<Result>,
}
pub trait TicTacToeReferee<const N: u32> {
    fn receive_move(&mut self, placement: &Placement, player: PlayerID) -> MoveResult;
}

#[derive(PartialEq, Clone)]
pub struct PlayerID {
    name: String,
    id: u32,
}

pub trait Player<const N: u32> {
    fn do_move(&mut self, board: &BoardState) -> Placement;
}

pub struct TicTacToeArena<const N: u32> {
    player1: Box<dyn Player<N>>,
    player2: Box<dyn Player<N>>,
    referee: Box<dyn TicTacToeReferee<N>>,
}
