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
        let mut tries = Board::new_from_existing(board, NSamplesT::from(0_u8));
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
                            has_win_prob |= player_id == self.id;
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

        let has_draw = draws.iter_2d().any(|(_, count)| *count > 0);
        let working_arr = if has_win_prob {
            wins
        } else if has_draw {
            draws
        } else {
            tries.clone()
        };
        let mut placements = Placement::new_from_existing(board, 0.0_f32);
        let placements_iter =
            working_arr
                .joint_into_iter_2d(tries)
                .map(|(pp, count, nof_tries)| {
                    let chance = if nof_tries == 0 {
                        0.0
                    } else {
                        f32::from(count) / f32::from(nof_tries)
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
    use crate::player::guessing::GuessingPlayer;
    use crate::referee::NaiveReferee;
    use rstest::*;

    #[rstest]
    #[case(
    GameState::new_with_values([[Some(0), None], [None, None]]).unwrap(), 2
    )]
    #[case(
        GameState::new_with_values([
            [Some(0), Some(0), None],
            [Some(0), Some(1), None],
            [None, None, Some(1)],
        ]).unwrap(), 3
    )]
    fn chooses_a_legal_move_when_all_simulations_are_losses(
        #[case] board: GameState,
        #[case] winning_length: u8,
    ) {
        let mut player0 = GuessingPlayer { id: 0 };
        let mut player1 = GuessingPlayer { id: 1 };
        let mut referee = NaiveReferee::new(winning_length);
        let mut player =
            CountBoundMCTSPlayer::new(1, 10, &mut player0, &mut player1, &mut referee);
        let placements = player.do_move(&board);
        assert!(
            board
                .joint_iter_2d(&placements)
                .any(|(_, cell, weight)| cell.is_free() && *weight > 0.0)
        );
    }
}
