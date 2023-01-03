pub type Placement<const N: usize> = [[f32; N]; N];

pub type PlayerID = usize;
pub type BoardStateEntry = Option<PlayerID>;

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
pub trait TicTacToeReferee<const N: usize> {
    fn receive_move(
        &mut self,
        board: &mut Board<N>,
        placement: &PointPlacement,
        player: PlayerID,
    ) -> Option<Result>;
}

pub trait Player<const N: usize> {
    fn do_move(&mut self, board: &Board<N>) -> &Placement<N>;
    fn get_id(&self) -> PlayerID;
}

pub struct TicTacToeArena<const N: usize> {
    active_player: usize,
    board: Board<N>,
    players: [Box<dyn Player<N>>; 2],
    referee: Box<dyn TicTacToeReferee<N>>,
}

impl<const N: usize> TicTacToeArena<N> {
    pub fn do_next_move(&mut self) -> Option<Result> {
        let cur_player = &mut self.players[self.active_player % 2];
        self.active_player += 1;
        let placements = cur_player.do_move(&self.board);
        let point_placement = PointPlacement { row: 0, col: 0 };
        let result =
            self.referee
                .receive_move(&mut self.board, &point_placement, cur_player.get_id());
        return result;
    }

    pub fn get_board(&self) -> Board<N> {
        self.board.clone()
    }
}
