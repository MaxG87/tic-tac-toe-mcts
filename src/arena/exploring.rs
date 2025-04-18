use crate::interfaces::{
    GameState, Placement, Player, PlayerID, PointPlacement, Result, TicTacToeArena,
    TicTacToeReferee,
};
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use rand::rng;

pub struct ExploringTicTacToeArena<'arena> {
    active_player: usize,
    board: GameState,
    players: [&'arena mut (dyn Player); 2],
    referee: &'arena mut (dyn TicTacToeReferee),
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
    fn do_next_move(&mut self) -> (Result, PlayerID, Option<PointPlacement>) {
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
            None => (Result::Defeat, cur_player.get_id(), None),
        }
    }

    fn get_board(&self) -> GameState {
        self.board.clone()
    }
}
