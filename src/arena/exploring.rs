use crate::interfaces::{
    GameResult, GameState, Placement, Player, PlayerID, PointPlacement, TicTacToeArena,
    TicTacToeReferee,
};
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use rand::rng;

pub struct ExploringTicTacToeArena<'arena> {
    active_player: usize,
    board: GameState,
    players: [&'arena mut dyn Player; 2],
    referee: &'arena mut dyn TicTacToeReferee,
}

impl<'arena> ExploringTicTacToeArena<'arena> {
    pub fn new(
        board: GameState,
        players: [&'arena mut dyn Player; 2],
        starting_player: PlayerID,
        referee: &'arena mut dyn TicTacToeReferee,
    ) -> Self {
        let matching_players: Vec<_> = players
            .iter()
            .enumerate()
            .filter_map(|(n, cur)| {
                if cur.get_id() == starting_player {
                    Some(n)
                } else {
                    None
                }
            })
            .collect();

        match matching_players[..] {
            [] => panic!("No matching player found for ID {starting_player}"),
            [n] => Self {
                board,
                players,
                active_player: n,
                referee,
            },
            _ => panic!("Multiple matching player found for ID {starting_player}"),
        }
    }

    fn sample_point_placement(
        board: &GameState,
        placement: Placement,
    ) -> Option<PointPlacement> {
        let mut pps = Vec::<PointPlacement>::new();
        let mut weights = Vec::<f32>::new();

        // Get point placement candidates with weights
        for (pp, weight) in placement.into_iter_2d() {
            if board[pp].is_taken() {
                // Skip already occupied cells
                continue;
            }
            if weight == 0.0 {
                continue;
            }
            pps.push(pp);
            weights.push(weight);
        }

        if weights.is_empty() {
            return None;
        }

        // Sample candidate from eligble options
        let mut rng = rng();
        let dist = WeightedIndex::new(weights).unwrap();
        let sampled_idx = dist.sample(&mut rng);
        Some(pps[sampled_idx])
    }
}

impl TicTacToeArena for ExploringTicTacToeArena<'_> {
    fn do_next_move(&mut self) -> (GameResult, PlayerID, Option<PointPlacement>) {
        let cur_player = &mut self.players[self.active_player % 2];
        self.active_player += 1;
        let placements = cur_player.do_move(&self.board);
        let maybe_pp =
            ExploringTicTacToeArena::sample_point_placement(&self.board, placements);

        match maybe_pp {
            Some(pp) => {
                let result =
                    self.referee
                        .receive_move(&mut self.board, pp, cur_player.get_id());
                (result, cur_player.get_id(), Some(pp))
            }
            None => (GameResult::Defeat, cur_player.get_id(), None),
        }
    }

    fn get_board(&self) -> GameState {
        self.board.clone()
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
    fn samples_the_only_legal_positive_weight() {
        let board =
            GameState::new_with_values([[Some(1), None], [None, None]]).unwrap();
        let mut player0 = FixedPlayer {
            id: 0,
            placements: Placement::new_with_values([[100.0_f32, 0.0], [0.0, 1.0]])
                .unwrap(),
        };
        let mut player1 = FixedPlayer {
            id: 1,
            placements: Placement::new_with_values([[0.0_f32, 0.0], [0.0, 1.0]])
                .unwrap(),
        };
        let mut referee = NaiveReferee::new(3);
        let mut arena = ExploringTicTacToeArena::new(
            board,
            [&mut player0, &mut player1],
            0,
            &mut referee,
        );

        assert_eq!(
            arena.do_next_move(),
            (
                GameResult::Undecided,
                0,
                Some(PointPlacement { row: 1, column: 1 })
            )
        );
        assert_eq!(
            arena.get_board()[PointPlacement { row: 1, column: 1 }],
            Some(0).into()
        );
    }

    #[test]
    fn no_legal_positive_weight_results_in_defeat_without_a_move() {
        let board = GameState::new(2, 2, None);
        let mut player0 = FixedPlayer {
            id: 0,
            placements: Placement::new(2, 2, 0.0_f32),
        };
        let mut player1 = FixedPlayer {
            id: 1,
            placements: Placement::new(2, 2, 0.0_f32),
        };
        let mut referee = NaiveReferee::new(3);
        let mut arena = ExploringTicTacToeArena::new(
            board.clone(),
            [&mut player0, &mut player1],
            0,
            &mut referee,
        );

        assert_eq!(arena.do_next_move(), (GameResult::Defeat, 0, None));
        assert_eq!(arena.get_board(), board);
    }
}
