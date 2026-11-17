use crate::arena::exploring::ExploringTicTacToeArena;
use crate::board::Board;
use crate::interfaces::{
    GameResult, GameState, Placement, Player, PlayerID, PointPlacement, TicTacToeArena,
    TicTacToeReferee,
};

type NSamplesT = u16;

pub struct CountBoundMCTSPlayer<'player> {
    id: PlayerID,
    nsamples: NSamplesT,
    player0: &'player mut dyn Player,
    player1: &'player mut dyn Player,
    referee: &'player mut dyn TicTacToeReferee,
}
impl<'player> CountBoundMCTSPlayer<'player> {
    #[allow(dead_code)]
    pub fn new(
        id: PlayerID,
        nsamples: NSamplesT,
        player0: &'player mut dyn Player,
        player1: &'player mut dyn Player,
        referee: &'player mut dyn TicTacToeReferee,
    ) -> Self {
        Self {
            id,
            nsamples,
            player0,
            player1,
            referee,
        }
    }
}
impl Player for CountBoundMCTSPlayer<'_> {
    fn do_move(&mut self, board: &GameState) -> Placement {
        let mut tries = Board::<NSamplesT>::new_from_existing(board, 0 as NSamplesT);
        let mut wins = tries.clone();
        let mut draws = tries.clone();
        let mut has_win_prob = false;

        for _ in 0..self.nsamples {
            let mut my_arena = ExploringTicTacToeArena::new(
                board.clone(),
                [&mut *self.player0, &mut *self.player1],
                self.id,
                &mut *self.referee,
            );

            let (result, player_id, first_point_placement) =
                CountBoundMCTSPlayer::do_one_step_sample(&mut my_arena);

            match first_point_placement {
                Some(pp) => {
                    tries[pp] += 1;
                    match result {
                        GameResult::Victory => {
                            wins[pp] += NSamplesT::from(player_id == self.id);
                            has_win_prob |= true;
                        }
                        GameResult::Draw => {
                            draws[pp] += 1;
                        }
                        _ => {}
                    }
                }
                None => panic!("No legal move was made!"),
            }
        }
        println!("{wins:?}");
        println!("{draws:?}");
        println!("{tries:?}");

        let working_arr = if has_win_prob { wins } else { draws };
        let mut placements = Placement::new_from_existing(board, 0.0_f32);
        let placements_iter =
            working_arr
                .joint_into_iter_2d(tries)
                .map(|(pp, count, total)| {
                    let chance = if total == 0 {
                        0.0
                    } else {
                        f32::from(count) / f32::from(total)
                    };
                    (pp, chance)
                });
        for (pp, value) in placements_iter {
            placements[pp] = value;
        }

        println!("{placements:?}");
        placements
    }

    fn get_id(&self) -> PlayerID {
        self.id
    }
}

impl CountBoundMCTSPlayer<'_> {
    fn do_one_step_sample(
        arena: &mut ExploringTicTacToeArena,
    ) -> (GameResult, PlayerID, Option<PointPlacement>) {
        let (result, first_player_id, first_point_placement) = arena.do_next_move();
        if result != GameResult::Undecided {
            return (result, first_player_id, first_point_placement);
        }
        loop {
            match arena.do_next_move() {
                (GameResult::Undecided, _, _) => {}
                (result, player_id, _) => {
                    return (result, player_id, first_point_placement);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::fixed::FixedPlayer;
    use crate::referee::NaiveReferee;
    use crate::test_utils::{board, placement};
    use rstest::rstest;
    use std::collections::VecDeque;

    struct ScriptedPlayer {
        id: PlayerID,
        moves: VecDeque<Placement>,
    }

    impl Player for ScriptedPlayer {
        fn do_move(&mut self, _board: &GameState) -> Placement {
            self.moves.pop_front().expect("unexpected rollout move")
        }

        fn get_id(&self) -> PlayerID {
            self.id
        }
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 0)]
    fn prefers_a_sampled_draw_when_only_the_opponent_has_a_win(
        #[case] self_id: PlayerID,
        #[case] other_id: PlayerID,
    ) {
        let board = board(vec![
            vec![Some(self_id), Some(self_id), Some(other_id)],
            vec![Some(other_id), Some(other_id), Some(self_id)],
            vec![None, None, None],
        ]);
        let mut rollout_player = ScriptedPlayer {
            id: self_id,
            moves: VecDeque::from([
                placement(vec![vec![0.0; 3], vec![0.0; 3], vec![1.0, 0.0, 0.0]]),
                placement(vec![vec![0.0; 3], vec![0.0; 3], vec![0.0, 0.0, 1.0]]),
                placement(vec![vec![0.0; 3], vec![0.0; 3], vec![0.0, 1.0, 0.0]]),
            ]),
        };
        let mut opponent = FixedPlayer::new(
            other_id,
            placement(vec![vec![0.0; 3], vec![0.0; 3], vec![1.0, 1.0, 0.0]]),
        );
        let mut referee = NaiveReferee::new(3);
        let mut mcts = CountBoundMCTSPlayer::new(
            self_id,
            2,
            &mut rollout_player,
            &mut opponent,
            &mut referee,
        );

        // The first rollout blocks the opponent and draws; the second allows a win.
        // With no wins for this player, draw probabilities should be used.
        let placements = mcts.do_move(&board);

        assert_eq!(
            placements,
            placement(vec![vec![0.0; 3], vec![0.0; 3], vec![1.0, 0.0, 0.0]])
        );
    }

    // The MCTS player is deterministic whenever both rollout players have a single
    // eligible cell per move, or when every sampled first move leads to the same
    // outcome.
    #[rstest]
    // winning first move is rated 1, everything else 0
    #[case(
        vec![vec![None, Some(0)], vec![None, None]],
        0, 2, 1,
        vec![vec![1.0, 0.0], vec![0.0, 0.0]],
        vec![vec![1.0, 1.0], vec![1.0, 1.0]],
        vec![vec![1.0, 0.0], vec![0.0, 0.0]],
    )]
    // the same holds when the MCTS player is the second player
    #[case(
        vec![vec![None, Some(1)], vec![None, None]],
        1, 2, 1,
        vec![vec![1.0, 1.0], vec![1.0, 1.0]],
        vec![vec![1.0, 0.0], vec![0.0, 0.0]],
        vec![vec![1.0, 0.0], vec![0.0, 0.0]],
    )]
    // without any victory draws are used: every first move leads to a draw
    #[case(
        vec![vec![None, None]],
        0, 3, 100,
        vec![vec![1.0, 1.0]],
        vec![vec![1.0, 1.0]],
        vec![vec![1.0, 1.0]],
    )]
    // cells that were never tried get probability 0
    #[case(
        vec![vec![None, None]],
        0, 3, 10,
        vec![vec![1.0, 0.0]],
        vec![vec![0.0, 1.0]],
        vec![vec![1.0, 0.0]],
    )]
    fn rates_first_moves(
        #[case] initial_board: Vec<Vec<Option<PlayerID>>>,
        #[case] mcts_id: PlayerID,
        #[case] win_length: u16,
        #[case] nsamples: NSamplesT,
        #[case] weights0: Vec<Vec<f32>>,
        #[case] weights1: Vec<Vec<f32>>,
        #[case] expected: Vec<Vec<f32>>,
    ) {
        let mut player0 = FixedPlayer::new(0, placement(weights0));
        let mut player1 = FixedPlayer::new(1, placement(weights1));
        let mut referee = NaiveReferee::new(win_length);
        let mut mcts = CountBoundMCTSPlayer::new(
            mcts_id,
            nsamples,
            &mut player0,
            &mut player1,
            &mut referee,
        );

        let placements = mcts.do_move(&board(initial_board));

        assert_eq!(placements, placement(expected));
    }
}
