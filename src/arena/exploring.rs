use crate::interfaces::*;
use crate::utils::*;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;
use rand::rng;

pub struct ExploringTicTacToeArena<'arena, const N: BoardSizeT, const K: WinLengthT> {
    active_player: PlayerID,
    board: Board<N>,
    players: [&'arena mut (dyn Player<N, K>); 2],
    referee: &'arena mut (dyn TicTacToeReferee<N, K>),
}

impl<'arena, const N: BoardSizeT, const K: WinLengthT>
    ExploringTicTacToeArena<'arena, N, K>
{
    pub fn new(
        board: Board<N>,
        players: [&'arena mut dyn Player<N, K>; 2],
        starting_player: PlayerID,
        referee: &'arena mut dyn TicTacToeReferee<N, K>,
    ) -> Self {
        let matching_players: Vec<PlayerID> = players
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
        board: &Board<N>,
        placement: Placement<N>,
    ) -> Option<PointPlacement> {
        let mut pps = Vec::<PointPlacement>::new();
        let mut weights = Vec::<f32>::new();

        // Get point placement candidates with weights
        for (row, column, weight) in into_iter_2d_array(&placement) {
            let pp = PointPlacement { row, column };
            if board.has_placement_at(pp) {
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

impl<const N: BoardSizeT, const K: WinLengthT> TicTacToeArena<N, K>
    for ExploringTicTacToeArena<'_, N, K>
{
    fn do_next_move(&mut self) -> (Result, PlayerID, Option<PointPlacement>) {
        let cur_player = &mut self.players[self.active_player % 2];
        self.active_player += 1;
        let placements = cur_player.do_move(&self.board);
        let maybe_pp = ExploringTicTacToeArena::<N, K>::sample_point_placement(
            &self.board,
            placements,
        );

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

    fn get_board(&self) -> Board<N> {
        self.board.clone()
    }
}
