use crate::interfaces::*;

pub struct MinMaxPlayer<'player, const N: usize, const K: usize, const MAX_DEPTH: usize> {
    other_id: PlayerID,
    referee: &'player mut dyn TicTacToeReferee<N, K>,
    self_id: PlayerID,
}

impl<'player, const N: usize, const K: usize, const MAX_DEPTH: usize>
    MinMaxPlayer<'player, N, K, MAX_DEPTH>
{
    const DEFAULT_PLACEMENT: Placement<N> = [[1.0; N]; N];

    pub fn new(
        referee: &'player mut dyn TicTacToeReferee<N, K>,
        self_id: PlayerID,
        other_id: PlayerID,
    ) -> MinMaxPlayer<N, K, MAX_DEPTH> {
        return MinMaxPlayer {
            other_id,
            referee,
            self_id,
        };
    }

    fn get_evaluations(&mut self, board: &mut Board<N>) -> [[f32; N]; N] {
        let mut evaluations = [[-1.0; N]; N];
        for row in 0..N {
            for column in 0..N {
                let pp = PointPlacement { row, column };
                if board.has_placement_at(&pp) {
                    continue;
                }
                evaluations[row][column] = match self.referee.receive_move(board, &pp, self.self_id)
                {
                    Some(Result::Defeat) | Some(Result::IllegalMove) => -1.0,
                    Some(Result::Victory) => 1.0,
                    Some(Result::Draw) | None => 0.0,
                };
                board.board[row][column] = None;
            }
        }
        return evaluations;
    }
}

impl<'player, const N: usize, const K: usize, const MAX_DEPTH: usize> Player<N, K>
    for MinMaxPlayer<'player, N, K, MAX_DEPTH>
{
    fn do_move(&mut self, board: &Board<N>) -> Placement<N> {
        let mut placements: Placement<N> = [[0.0; N]; N];
        let mut board = board.clone();

        let evaluations = self.get_evaluations(&mut board);
        let max = evaluations
            .into_iter()
            .flat_map(|row| row.into_iter())
            .reduce(|accum, val| if accum >= val { accum } else { val })
            .unwrap();

        if max <= 0.0 {
            return MinMaxPlayer::<N, K, MAX_DEPTH>::DEFAULT_PLACEMENT;
        }
        for row in 0..N {
            for column in 0..N {
                if evaluations[row][column] == max {
                    placements[row][column] = 1.0;
                }
            }
        }
        return placements;
    }

    fn get_id(&self) -> PlayerID {
        return self.self_id;
    }
}
