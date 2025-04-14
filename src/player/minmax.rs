use crate::interfaces::*;
use crate::lib::*;
use std::iter::*;

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
        match args.max_depth {
            0 => panic!("Lookahead must be at least 1!"),
            1 => self.get_evaluations_1(board, args),
            _ => self.get_evaluations_n(board, args),
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
        for (row, column, elem) in iter_mut_2d_array(&mut evaluations) {
            let pp = PointPlacement { row, column };
            let old_board_val = board.board[row][column];
            let move_result = self.referee.receive_move(board, pp, args.self_id);
            *elem = match move_result {
                Result::Defeat | Result::IllegalMove => DEFEAT,
                Result::Victory => VICTORY,
                Result::Draw | Result::Undecided => DRAW,
            };
            board.board[row][column] = old_board_val;
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
        let flattened: Vec<(usize, usize, &mut f32, BoardStateEntry)> = zip(
            iter_mut_2d_array(&mut evaluations),
            into_iter_2d_array(&board.board),
        )
        .map(|(eval, board)| (eval.0, eval.1, eval.2, board.2))
        .collect();

        for (row, column, cur_evaluation, old_board_val) in flattened {
            let pp = PointPlacement { row, column };
            let move_result = self.referee.receive_move(board, pp, args.self_id);
            *cur_evaluation = match move_result {
                Result::Defeat | Result::IllegalMove => DEFEAT,
                Result::Victory => VICTORY,
                Result::Draw => DRAW,
                Result::Undecided => {
                    let pp_evaluations = self.get_evaluations(board, pass_down_args.clone());
                    -get_maximum(&pp_evaluations)
                }
            };
            board.board[row][column] = old_board_val;
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
