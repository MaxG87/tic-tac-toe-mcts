use crate::interfaces::*;
use crate::utils::*;
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

fn get_maximum<const N: usize>(evaluations: &Evaluation<N>) -> f32 {
    into_iter_2d_array(evaluations)
        .map(|(_, _, val)| val)
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
    ) -> Self {
        Self {
            max_depth,
            other_id,
            referee,
            self_id,
        }
    }

    fn get_evaluations(
        &mut self,
        board: &mut Board<N>,
        args: &GetEvaluationsArgs,
    ) -> Evaluation<N> {
        match args.max_depth {
            0 => panic!("Lookahead must be at least 1!"),
            1 => self.get_evaluations_1(board, args),
            _ => self.get_evaluations_n(board, args),
        }
    }

    fn to_placement(evaluations: &Evaluation<N>) -> Placement<N> {
        let max = get_maximum(evaluations);
        if max == DEFEAT {
            println!("Sure defeat detected. Using default placements.");
            return Self::DEFAULT_PLACEMENT;
        }

        let mut placements: Placement<N> = [[0.0; N]; N];
        for row in 0..N {
            for column in 0..N {
                if evaluations[row][column] == max {
                    placements[row][column] = 1.0;
                }
            }
        }
        placements
    }

    fn get_evaluations_1(
        &mut self,
        board: &mut Board<N>,
        args: &GetEvaluationsArgs,
    ) -> Evaluation<N> {
        let mut evaluations = [[DEFEAT; N]; N];
        let flattened: Vec<(usize, usize, &mut f32, BoardStateEntry)> = joint_iter_2d_arrays(
            iter_mut_2d_array(&mut evaluations),
            into_iter_2d_array(&board.board),
        )
        .collect();

        for (row, column, cur_evaluation, old_board_val) in flattened {
            let pp = PointPlacement { row, column };
            let move_result = self.referee.receive_move(board, pp, args.self_id);
            *cur_evaluation = match move_result {
                Result::Defeat | Result::IllegalMove => DEFEAT,
                Result::Victory => VICTORY,
                Result::Draw | Result::Undecided => DRAW,
            };
            board.set_placement_at(pp, old_board_val);
        }
        evaluations
    }

    fn get_evaluations_n(
        &mut self,
        board: &mut Board<N>,
        args: &GetEvaluationsArgs,
    ) -> Evaluation<N> {
        let mut evaluations = [[DEFEAT; N]; N];
        let pass_down_args = GetEvaluationsArgs {
            other_id: args.self_id,
            self_id: args.other_id,
            max_depth: args.max_depth - 1,
        };
        let flattened: Vec<(usize, usize, &mut f32, BoardStateEntry)> = joint_iter_2d_arrays(
            iter_mut_2d_array(&mut evaluations),
            into_iter_2d_array(&board.board),
        )
        .collect();

        for (row, column, cur_evaluation, old_board_val) in flattened {
            let pp = PointPlacement { row, column };
            let move_result = self.referee.receive_move(board, pp, args.self_id);
            *cur_evaluation = match move_result {
                Result::Defeat | Result::IllegalMove => DEFEAT,
                Result::Victory => VICTORY,
                Result::Draw => DRAW,
                Result::Undecided => {
                    let pp_evaluations = self.get_evaluations(board, &pass_down_args);
                    -get_maximum(&pp_evaluations)
                }
            };
            board.set_placement_at(pp, old_board_val);
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
        let evaluations = self.get_evaluations(&mut board, &args);
        Self::to_placement(&evaluations)
    }

    fn get_id(&self) -> PlayerID {
        self.self_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::referee::*;
    use rstest::*;

    #[rstest]
    // direct winning moves
    #[case(Board {
            board: [
                [None, Some(1), None, None, None],
                [None, Some(0), None, None, None],
                [None, None, Some(0), None, Some(0)],
                [None, Some(0), None, None, Some(1)],
                [None, Some(1), None, None, Some(1)],
            ],
        },
        [
            [1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0, 0.0, 0.0],
        ],
        1
    )]
    // indirect winning moves
    #[case(Board {
            board: [
                [None, None, Some(0), Some(1)],
                [None, Some(1), None, None],
                [Some(0), None, None, None],
                [Some(1), None, None, None],
            ]
        },
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ],
        3
    )]
    fn correct_moves_are_found<const N: usize>(
        #[case] board: Board<N>,
        #[case] expected: Placement<N>,
        #[case] lookahead: u32,
    ) {
        const K: usize = 3;
        let other_id: BoardStateEntry = Some(1);
        let self_id: BoardStateEntry = Some(0);

        let mut referee = NaiveReferee::<N, K> {};
        let mut player = MinMaxPlayer::<N, K> {
            max_depth: lookahead,
            self_id: self_id.unwrap(),
            other_id: other_id.unwrap(),
            referee: &mut referee,
        };

        let result = player.do_move(&board);
        assert_eq!(result, expected)
    }
}
