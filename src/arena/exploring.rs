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
    use crate::player::fixed::FixedPlayer;
    use crate::referee::NaiveReferee;
    use crate::test_utils::{board, placement, pp};
    use rstest::rstest;

    type Step = (GameResult, PlayerID, Option<PointPlacement>);

    // Only cases where exactly one cell is eligible per move are deterministic.
    #[rstest]
    // zero weights and occupied cells are never sampled
    #[case(
        vec![vec![Some(1), None], vec![None, None]],
        vec![vec![100.0, 0.0], vec![0.0, 1.0]],
        vec![vec![0.0, 0.0], vec![0.0, 1.0]],
        0, 3,
        vec![(GameResult::Undecided, 0, Some(pp(1, 1)))],
    )]
    // players alternate
    #[case(
        vec![vec![None, None, None]],
        vec![vec![1.0, 0.0, 0.0]],
        vec![vec![0.0, 0.0, 1.0]],
        0, 3,
        vec![
            (GameResult::Undecided, 0, Some(pp(0, 0))),
            (GameResult::Undecided, 1, Some(pp(0, 2))),
        ],
    )]
    // the starting player can be the second one in the players array
    #[case(
        vec![vec![None, None, None]],
        vec![vec![1.0, 0.0, 0.0]],
        vec![vec![0.0, 0.0, 1.0]],
        1, 3,
        vec![
            (GameResult::Undecided, 1, Some(pp(0, 2))),
            (GameResult::Undecided, 0, Some(pp(0, 0))),
        ],
    )]
    // a completed line is a victory
    #[case(
        vec![vec![Some(0), Some(0), None], vec![Some(1), Some(1), None]],
        vec![vec![0.0, 0.0, 1.0], vec![0.0, 0.0, 0.0]],
        vec![vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 1.0]],
        0, 3,
        vec![(GameResult::Victory, 0, Some(pp(0, 2)))],
    )]
    // the last free cell without a line is a draw
    #[case(
        vec![vec![Some(0), None]],
        vec![vec![0.0, 1.0]],
        vec![vec![0.0, 0.0]],
        0, 3,
        vec![(GameResult::Draw, 0, Some(pp(0, 1)))],
    )]
    // no weight on a free cell is a defeat without a move
    #[case(
        vec![vec![None, None], vec![None, None]],
        vec![vec![0.0, 0.0], vec![0.0, 0.0]],
        vec![vec![1.0, 1.0], vec![1.0, 1.0]],
        0, 3,
        vec![(GameResult::Defeat, 0, None)],
    )]
    // weights only on occupied cells is a defeat without a move
    #[case(
        vec![vec![Some(1), None]],
        vec![vec![1.0, 0.0]],
        vec![vec![0.0, 1.0]],
        0, 3,
        vec![(GameResult::Defeat, 0, None)],
    )]
    fn plays_scripted_moves(
        #[case] initial_board: Vec<Vec<Option<PlayerID>>>,
        #[case] weights0: Vec<Vec<f32>>,
        #[case] weights1: Vec<Vec<f32>>,
        #[case] starting_player: PlayerID,
        #[case] win_length: u16,
        #[case] expected: Vec<Step>,
    ) {
        let mut player0 = FixedPlayer::new(0, placement(weights0));
        let mut player1 = FixedPlayer::new(1, placement(weights1));
        let mut referee = NaiveReferee::new(win_length);
        let mut arena = ExploringTicTacToeArena::new(
            board(initial_board),
            [&mut player0, &mut player1],
            starting_player,
            &mut referee,
        );

        let steps: Vec<Step> = expected.iter().map(|_| arena.do_next_move()).collect();

        assert_eq!(steps, expected);
    }

    #[rstest]
    #[case(0, 1, 2)]
    #[case(0, 1, 7)]
    #[should_panic(expected = "No matching player")]
    fn rejects_unknown_starting_player(
        #[case] id0: PlayerID,
        #[case] id1: PlayerID,
        #[case] starting_player: PlayerID,
    ) {
        let mut player0 = FixedPlayer::new(id0, placement(vec![vec![1.0]]));
        let mut player1 = FixedPlayer::new(id1, placement(vec![vec![1.0]]));
        let mut referee = NaiveReferee::new(3);
        ExploringTicTacToeArena::new(
            board(vec![vec![None]]),
            [&mut player0, &mut player1],
            starting_player,
            &mut referee,
        );
    }

    #[rstest]
    #[case(0, 0, 0)]
    #[case(3, 3, 3)]
    #[should_panic(expected = "Multiple matching player")]
    fn rejects_ambiguous_starting_player(
        #[case] id0: PlayerID,
        #[case] id1: PlayerID,
        #[case] starting_player: PlayerID,
    ) {
        let mut player0 = FixedPlayer::new(id0, placement(vec![vec![1.0]]));
        let mut player1 = FixedPlayer::new(id1, placement(vec![vec![1.0]]));
        let mut referee = NaiveReferee::new(3);
        ExploringTicTacToeArena::new(
            board(vec![vec![None]]),
            [&mut player0, &mut player1],
            starting_player,
            &mut referee,
        );
    }
}
