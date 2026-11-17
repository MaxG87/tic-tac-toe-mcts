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
    use crate::referee::NaiveReferee;

    struct FixedPlayer {
        id: PlayerID,
        placements: Placement,
    }

    impl Player for FixedPlayer {
        fn do_move(&mut self, _board: &GameState) -> Placement {
            self.placements.clone()
        }

        fn get_id(&self) -> PlayerID {
            self.id
        }
    }

    #[test]
    fn assigns_a_win_to_the_first_move_that_completes_the_line() {
        let board =
            GameState::new_with_values([[None, Some(0)], [None, None]]).unwrap();
        let winning_placement = PointPlacement { row: 0, column: 0 };
        let mut player0 = FixedPlayer {
            id: 0,
            placements: Placement::new(2, 2, 0.0_f32),
        };
        player0.placements[winning_placement] = 1.0;
        let mut player1 = FixedPlayer {
            id: 1,
            placements: Placement::new(2, 2, 1.0_f32),
        };
        let mut referee = NaiveReferee::new(2);
        let mut mcts =
            CountBoundMCTSPlayer::new(0, 1, &mut player0, &mut player1, &mut referee);

        let placements = mcts.do_move(&board);

        for (pp, weight) in placements.iter_2d() {
            assert_eq!(*weight, if pp == winning_placement { 1.0 } else { 0.0 });
        }
    }
}
