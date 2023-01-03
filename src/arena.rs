use rand::distributions::*;
use rand::prelude::*;
use std::fmt;

pub type BoardStateEntry = Option<PlayerID>;
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

#[derive(PartialEq, Copy, Clone)]
pub struct PointPlacement {
    pub row: usize,
    pub column: usize,
}

impl fmt::Display for PointPlacement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PointPlacement({}, {})", self.row, self.column)
    }
}

#[derive(PartialEq)]
pub enum Result {
    Defeat,
    Draw,
    IllegalMove,
    Victory,
}
impl fmt::Display for Result {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Result::Defeat => write!(f, "Defeat"),
            Result::Draw => write!(f, "Draw"),
            Result::IllegalMove => write!(f, "IllegalMove"),
            Result::Victory => write!(f, "Victory"),
        }
    }
}

pub trait TicTacToeReferee<const N: usize, const K: usize> {
    fn receive_move(
        &mut self,
        board: &mut Board<N>,
        placement: &PointPlacement,
        player: PlayerID,
    ) -> Option<Result>;
}

pub trait Player<const N: usize, const K: usize> {
    fn do_move(&mut self, board: &Board<N>) -> Placement<N>;
    fn get_id(&self) -> PlayerID;
}

pub struct TicTacToeArena<'arena, const N: usize, const K: usize> {
    active_player: usize,
    board: Board<N>,
    players: [&'arena mut (dyn Player<N, K>); 2],
    referee: &'arena mut (dyn TicTacToeReferee<N, K>),
}

impl<'arena, const N: usize, const K: usize> TicTacToeArena<'arena, N, K> {
    pub fn new(
        board: Board<N>,
        players: [&'arena mut dyn Player<N, K>; 2],
        referee: &'arena mut dyn TicTacToeReferee<N, K>,
    ) -> TicTacToeArena<'arena, N, K> {
        TicTacToeArena {
            active_player: 0,
            board,
            players,
            referee,
        }
    }

    pub fn do_next_move(&mut self) -> (Option<Result>, PlayerID, Option<PointPlacement>) {
        let cur_player = &mut self.players[self.active_player % 2];
        self.active_player += 1;
        let placements = cur_player.do_move(&self.board);
        let maybe_point_placement =
            TicTacToeArena::<N, K>::sample_point_placement(&self.board, &placements);

        match maybe_point_placement {
            Some(point_placement) => {
                let maybe_result = self.referee.receive_move(
                    &mut self.board,
                    &point_placement,
                    cur_player.get_id(),
                );
                return (maybe_result, cur_player.get_id(), Some(point_placement));
            }
            None => {
                return (Some(Result::Defeat), cur_player.get_id(), None);
            }
        }
    }

    pub fn get_board(&self) -> Board<N> {
        self.board.clone()
    }

    fn sample_point_placement(
        board: &Board<N>,
        placement: &Placement<N>,
    ) -> Option<PointPlacement> {
        let mut point_placements = Vec::<PointPlacement>::new();
        let mut weights = Vec::<f32>::new();

        // Get point placement candidates with weights
        for row in 0..board.rows() {
            for column in 0..board.columns() {
                let maybe_id = &board.board[row][column];
                let weight = placement[row][column];
                if let Some(_) = maybe_id {
                    continue;
                } else if weight == 0.0 {
                    continue;
                } else {
                    point_placements.push(PointPlacement { row, column });
                    weights.push(weight);
                }
            }
        }

        if weights.len() == 0 {
            return None;
        }

        // Sample candidate from eligble options
        let mut rng = thread_rng();
        let dist = WeightedIndex::new(weights).unwrap();
        let sampled_idx = dist.sample(&mut rng);
        return Some(point_placements[sampled_idx]);
    }
}
