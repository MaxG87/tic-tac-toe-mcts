use crate::interfaces::*;
use rand::distributions::*;
use rand::prelude::*;

pub struct ExploringTicTacToeArena<'arena, const N: usize, const K: usize> {
    active_player: usize,
    board: Board<N>,
    players: [&'arena mut (dyn Player<N, K>); 2],
    referee: &'arena mut (dyn TicTacToeReferee<N, K>),
}

impl<'arena, const N: usize, const K: usize> ExploringTicTacToeArena<'arena, N, K> {
    pub fn new(
        board: Board<N>,
        players: [&'arena mut dyn Player<N, K>; 2],
        active_player: PlayerID,
        referee: &'arena mut dyn TicTacToeReferee<N, K>,
    ) -> ExploringTicTacToeArena<'arena, N, K> {
        let mut maybe_startid: Option<usize> = None;
        for n in 0..players.len() {
            if players[n].get_id() == active_player {
                maybe_startid = Some(n);
            }
        }

        match maybe_startid {
            Some(n) => {
                return ExploringTicTacToeArena {
                    board,
                    players,
                    active_player: n,
                    referee,
                }
            }
            None => {
                panic!("No matching player found for ID {active_player}");
            }
        }
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

impl<'arena, const N: usize, const K: usize> TicTacToeArena<N, K>
    for ExploringTicTacToeArena<'arena, N, K>
{
    fn do_next_move(&mut self) -> (Option<Result>, PlayerID, Option<PointPlacement>) {
        let cur_player = &mut self.players[self.active_player % 2];
        self.active_player += 1;
        let placements = cur_player.do_move(&self.board);
        let maybe_point_placement =
            ExploringTicTacToeArena::<N, K>::sample_point_placement(&self.board, &placements);

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

    fn get_board(&self) -> Board<N> {
        self.board.clone()
    }
}
