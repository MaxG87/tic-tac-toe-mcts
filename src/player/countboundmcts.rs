use crate::arena::exploring::*;
use crate::interfaces::*;

pub struct CountBoundMCTSPlayer<'player, const N: BoardSizeT, const K: WinLengthT> {
    id: PlayerID,
    nsamples: u32,
    player0: &'player mut dyn Player<N, K>,
    player1: &'player mut dyn Player<N, K>,
    referee: &'player mut dyn TicTacToeReferee<N, K>,
}
impl<'player, const N: BoardSizeT, const K: WinLengthT>
    CountBoundMCTSPlayer<'player, N, K>
{
    #[allow(dead_code)]
    pub fn new(
        id: PlayerID,
        nsamples: u32,
        player0: &'player mut dyn Player<N, K>,
        player1: &'player mut dyn Player<N, K>,
        referee: &'player mut dyn TicTacToeReferee<N, K>,
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
impl<const N: BoardSizeT, const K: WinLengthT> Player<N, K>
    for CountBoundMCTSPlayer<'_, N, K>
{
    fn do_move(&mut self, board: &Board<N>) -> Placement<N> {
        let mut tries = [[0u32; N]; N];
        let mut wins = [[0u32; N]; N];
        let mut draws = [[0u32; N]; N];
        let mut has_win_prob = false;

        for _ in 0..self.nsamples {
            let mut my_arena = ExploringTicTacToeArena::<N, K>::new(
                board.clone(),
                [&mut *self.player0, &mut *self.player1],
                self.id,
                &mut *self.referee,
            );

            let (result, player_id, first_point_placement) =
                CountBoundMCTSPlayer::do_one_step_sample(&mut my_arena);

            match first_point_placement {
                Some(pp) => {
                    tries[pp.row][pp.column] += 1;
                    match result {
                        Result::Victory => {
                            wins[pp.row][pp.column] += u32::from(player_id == self.id);
                            has_win_prob |= true;
                        }
                        Result::Draw => {
                            draws[pp.row][pp.column] += 1;
                        }
                        _ => {}
                    }
                }
                None => panic!("No legal move was made!"),
            }
        }
        let mut placements: Placement<N> = [[0f32; N]; N];
        let working_arr = if has_win_prob { &wins } else { &draws };
        for row in 0..N {
            for column in 0..N {
                placements[row][column] = if tries[row][column] == 0 {
                    0.0
                } else {
                    (working_arr[row][column] as f32) / (tries[row][column] as f32)
                };
            }
        }

        println!("{wins:?}");
        println!("{draws:?}");
        println!("{tries:?}");
        println!("{placements:?}");
        placements
    }

    fn get_id(&self) -> PlayerID {
        self.id
    }
}

impl<const N: BoardSizeT, const K: WinLengthT> CountBoundMCTSPlayer<'_, N, K> {
    fn do_one_step_sample(
        arena: &mut ExploringTicTacToeArena<N, K>,
    ) -> (Result, PlayerID, Option<PointPlacement>) {
        let (result, first_player_id, first_point_placement) = arena.do_next_move();
        if result != Result::Undecided {
            return (result, first_player_id, first_point_placement);
        }
        loop {
            match arena.do_next_move() {
                (Result::Undecided, _, _) => continue,
                (result, player_id, _) => {
                    return (result, player_id, first_point_placement);
                }
            }
        }
    }
}
