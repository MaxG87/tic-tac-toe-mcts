use crate::game_state_storage::GameStateStorage;
use crate::interfaces::{
    Evaluation, GameState, Placement, Player, PlayerID, Result, TicTacToeReferee,
};
use std::iter::Iterator;

const DEFEAT: f32 = -1.0;
const VICTORY: f32 = 1.0;
const DRAW: f32 = 0.0;

#[derive(Debug, Clone)]
struct GetEvaluationsArgs {
    self_id: PlayerID,
    other_id: PlayerID,
    max_depth: u32,
}

pub struct MinMaxPlayer<'player> {
    max_depth: u32,
    other_id: PlayerID,
    game_state_storage: &'player mut dyn GameStateStorage<GameState, Evaluation>,
    referee: &'player mut dyn TicTacToeReferee,
    self_id: PlayerID,
}

fn get_maximum(evaluations: &Evaluation) -> f32 {
    let max = evaluations
        .iter_2d()
        .map(|(_, val)| val)
        .reduce(|accum, val| if accum > val { accum } else { val })
        .unwrap();
    *max
}

impl<'player> MinMaxPlayer<'player> {
    pub fn new(
        max_depth: u32,
        other_id: PlayerID,
        game_state_storage: &'player mut dyn GameStateStorage<GameState, Evaluation>,
        referee: &'player mut dyn TicTacToeReferee,
        self_id: PlayerID,
    ) -> Self {
        Self {
            max_depth,
            other_id,
            game_state_storage,
            referee,
            self_id,
        }
    }

    fn get_evaluations(
        &mut self,
        board: &mut GameState,
        args: &GetEvaluationsArgs,
    ) -> Evaluation {
        if let Some(evaluations) =
            self.game_state_storage.get_payload(board, args.max_depth)
        {
            return evaluations.to_owned();
        }
        let evaluations = match args.max_depth {
            0 => panic!("Lookahead must be at least 1!"),
            1 => self.get_evaluations_1(board, args),
            _ => self.get_evaluations_n(board, args),
        };
        self.game_state_storage.register_game_state(
            board,
            evaluations.clone(),
            args.max_depth,
        );
        evaluations
    }

    fn to_placement(evaluations: &Evaluation) -> Placement {
        let max = get_maximum(evaluations);
        if max == DEFEAT {
            println!("Sure defeat detected. Using default placements.");
            return Placement::new(
                evaluations.get_number_of_rows(),
                evaluations.get_number_of_columns(),
                1.0,
            );
        }

        let mut placements = Placement::new_from_existing(evaluations, 0.0);
        for (pp, eval) in evaluations.iter_2d() {
            // Direct comparsion is fine as float value was taken from the
            // evaluations array.
            #[allow(clippy::float_cmp)]
            if *eval == max {
                placements[pp] = 1.0;
            }
        }
        placements
    }

    fn get_evaluations_1(
        &mut self,
        board: &GameState,
        args: &GetEvaluationsArgs,
    ) -> Evaluation {
        let mut evaluation = Evaluation::new(
            board.get_number_of_rows(),
            board.get_number_of_columns(),
            DEFEAT,
        );
        let mut temporary_board = board.clone();

        for (pp, old_board_val) in board.iter_2d() {
            let move_result =
                self.referee
                    .receive_move(&mut temporary_board, pp, args.self_id);
            evaluation[pp] = match move_result {
                Result::Defeat | Result::IllegalMove => DEFEAT,
                Result::Victory => VICTORY,
                Result::Draw | Result::Undecided => DRAW,
            };
            temporary_board[pp] = *old_board_val;
        }
        evaluation
    }

    fn get_evaluations_n(
        &mut self,
        board: &GameState,
        args: &GetEvaluationsArgs,
    ) -> Evaluation {
        let mut evaluation = Evaluation::new(
            board.get_number_of_rows(),
            board.get_number_of_columns(),
            DEFEAT,
        );
        let mut temporary_board = board.clone();
        let pass_down_args = GetEvaluationsArgs {
            other_id: args.self_id,
            self_id: args.other_id,
            max_depth: args.max_depth - 1,
        };

        for (pp, old_board_val) in board.iter_2d() {
            let move_result =
                self.referee
                    .receive_move(&mut temporary_board, pp, args.self_id);
            evaluation[pp] = match move_result {
                Result::Defeat | Result::IllegalMove => DEFEAT,
                Result::Victory => VICTORY,
                Result::Draw => DRAW,
                Result::Undecided => {
                    let pp_evaluations =
                        self.get_evaluations(&mut temporary_board, &pass_down_args);
                    -get_maximum(&pp_evaluations)
                }
            };
            temporary_board[pp] = *old_board_val;
        }

        if args.max_depth == self.max_depth {
            println!("{evaluation:?}");
        }
        evaluation
    }
}

impl Player for MinMaxPlayer<'_> {
    fn do_move(&mut self, board: &GameState) -> Placement {
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
    use crate::game_state_storage::NaiveGameStateStorage;
    use crate::referee::*;
    use rstest::*;

    #[rstest]
    // direct winning moves
    #[case(GameState::new_with_values(
        [
            [None, Some(1), None, None, None],
            [None, Some(0), None, None, None],
            [None, None, Some(0), None, Some(0)],
            [None, Some(0), None, None, Some(1)],
            [None, Some(1), None, None, Some(1)],
        ]
    ).unwrap(),
        Placement::new_with_values([
            [1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0, 0.0, 0.0],
        ]).unwrap(),
        1
    )]
    // indirect winning moves
    #[case(GameState::new_with_values(
            [
                [None, None, Some(0), Some(1)],
                [None, Some(1), None, None],
                [Some(0), None, None, None],
                [Some(1), None, None, None]
            ],

        ).unwrap(),
        Placement::new_with_values([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]).unwrap(),
        3
    )]
    fn correct_moves_are_found(
        #[case] board: GameState,
        #[case] expected: Placement,
        #[case] lookahead: u32,
    ) {
        let winning_length = 3;
        let other_id = 1;
        let self_id = 0;
        let mut game_state_storage = NaiveGameStateStorage::<_, _>::new();

        let mut referee = NaiveReferee::new(winning_length);
        let mut player = MinMaxPlayer {
            max_depth: lookahead,
            self_id,
            other_id,
            game_state_storage: &mut game_state_storage,
            referee: &mut referee,
        };

        let result = player.do_move(&board);
        let result_from_storage =
            game_state_storage.get_payload(&board, lookahead).unwrap();
        let placement_from_storage = MinMaxPlayer::to_placement(result_from_storage);
        assert_eq!(placement_from_storage, expected);
        assert_eq!(result, expected);
    }
}
