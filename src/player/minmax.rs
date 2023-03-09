use crate::interfaces::*;

const DEFEAT: f32 = -1.0;
const VICTORY: f32 = 1.0;
const DRAW: f32 = 0.0;

#[derive(Debug, Clone)]
struct GetEvaluationsArgs {
    self_id: PlayerID,
    other_id: PlayerID,
    max_depth: u32,
}

pub struct MinMaxPlayer<'player, const N: usize, const K: usize> {
    max_depth: u32,
    other_id: PlayerID,
    referee: &'player mut dyn TicTacToeReferee<N, K>,
    self_id: PlayerID,
}

fn get_maximum<const N: usize>(evaluations: &Evaluation<N>) -> &f32 {
    evaluations
        .iter()
        .flat_map(|row| row.iter())
        .reduce(|accum, val| if accum > val { accum } else { val })
        .unwrap()
}

impl<'player, const N: usize, const K: usize> MinMaxPlayer<'player, N, K> {
    const DEFAULT_PLACEMENT: Placement<N> = [[1.0; N]; N];

    pub fn new(
        max_depth: u32,
        other_id: PlayerID,
        referee: &'player mut dyn TicTacToeReferee<N, K>,
        self_id: PlayerID,
    ) -> MinMaxPlayer<N, K> {
        MinMaxPlayer {
            other_id,
            self_id,
            max_depth,
            referee,
        }
    }

    fn get_evaluations(&mut self, board: &mut Board<N>, args: GetEvaluationsArgs) -> Evaluation<N> {
        if args.max_depth < 1 {
            panic!("Lookahead must be at least 1!")
        } else if args.max_depth == 1 {
            self.get_evaluations_1(board, args)
        } else {
            self.get_evaluations_n(board, args)
        }
    }

    fn to_placement(&self, evaluations: &Evaluation<N>) -> Placement<N> {
        let max = get_maximum(evaluations);
        if max == &DEFEAT {
            println!("Sure defeat detected. Using default placements.");
            return MinMaxPlayer::<N, K>::DEFAULT_PLACEMENT;
        }

        let mut placements: Placement<N> = [[0.0; N]; N];
        for row in 0..N {
            for column in 0..N {
                if evaluations[row][column] == *max {
                    placements[row][column] = 1.0;
                }
            }
        }
        placements
    }

    fn get_evaluations_1(
        &mut self,
        board: &mut Board<N>,
        args: GetEvaluationsArgs,
    ) -> Evaluation<N> {
        let mut evaluations = [[DEFEAT; N]; N];
        for row in 0..N {
            for column in 0..N {
                let pp = PointPlacement { row, column };
                let move_result = self.referee.receive_move(board, pp, args.self_id);
                evaluations[row][column] = match move_result {
                    Some(Result::Defeat) | Some(Result::IllegalMove) => DEFEAT,
                    Some(Result::Victory) => VICTORY,
                    Some(Result::Draw) | None => DRAW,
                };
                if move_result != Some(Result::IllegalMove) {
                    board.board[row][column] = None;
                }
            }
        }
        evaluations
    }

    fn get_evaluations_n(
        &mut self,
        board: &mut Board<N>,
        args: GetEvaluationsArgs,
    ) -> Evaluation<N> {
        let mut evaluations = [[DEFEAT; N]; N];
        let pass_down_args = GetEvaluationsArgs {
            other_id: args.self_id,
            self_id: args.other_id,
            max_depth: args.max_depth - 1,
        };
        for row in 0..N {
            for column in 0..N {
                let pp = PointPlacement { row, column };
                let move_result = self.referee.receive_move(board, pp, args.self_id);
                evaluations[row][column] = match move_result {
                    Some(Result::Defeat) | Some(Result::IllegalMove) => DEFEAT,
                    Some(Result::Victory) => VICTORY,
                    Some(Result::Draw) => DRAW,
                    None => {
                        let pp_evaluations = self.get_evaluations(board, pass_down_args.clone());
                        -get_maximum(&pp_evaluations)
                    }
                };
                if move_result != Some(Result::IllegalMove) {
                    board.board[row][column] = None;
                }
            }
        }
        if args.max_depth == self.max_depth {
            println!("{evaluations:?}");
        }
        evaluations
    }
}

impl<'player, const N: usize, const K: usize> Player<N, K> for MinMaxPlayer<'player, N, K> {
    fn do_move(&mut self, board: &Board<N>) -> Placement<N> {
        let mut board = board.clone();
        let args = GetEvaluationsArgs {
            self_id: self.self_id,
            other_id: self.other_id,
            max_depth: self.max_depth,
        };
        let evaluations = self.get_evaluations(&mut board, args);
        self.to_placement(&evaluations)
    }

    fn get_id(&self) -> PlayerID {
        self.self_id
    }
}
