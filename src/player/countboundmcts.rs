use crate::arena::exploring::ExploringTicTacToeArena;
use crate::board::Board;
use crate::interfaces::{
    BoardSizeT, Placement, Player, PlayerID, PointPlacement, Result, TicTacToeArena,
    TicTacToeReferee, WinLengthT,
};

type NSamplesT = u16;

pub struct CountBoundMCTSPlayer<'player, const N: BoardSizeT, const K: WinLengthT> {
    id: PlayerID,
    nsamples: NSamplesT,
    player0: &'player mut dyn Player<N, K>,
    player1: &'player mut dyn Player<N, K>,
    referee: &'player mut dyn TicTacToeReferee<K>,
}
impl<'player, const N: BoardSizeT, const K: WinLengthT>
    CountBoundMCTSPlayer<'player, N, K>
{
    #[allow(dead_code)]
    pub fn new(
        id: PlayerID,
        nsamples: NSamplesT,
        player0: &'player mut dyn Player<N, K>,
        player1: &'player mut dyn Player<N, K>,
        referee: &'player mut dyn TicTacToeReferee<K>,
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
    fn do_move(&mut self, board: &Board) -> Placement<N> {
        let mut tries: [[NSamplesT; N]; N] = [[0; N]; N];
        let mut wins: [[NSamplesT; N]; N] = [[0; N]; N];
        let mut draws: [[NSamplesT; N]; N] = [[0; N]; N];
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
                            wins[pp.row][pp.column] +=
                                NSamplesT::from(player_id == self.id);
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
                    f32::from(working_arr[row][column]) / f32::from(tries[row][column])
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
                (Result::Undecided, _, _) => {}
                (result, player_id, _) => {
                    return (result, player_id, first_point_placement);
                }
            }
        }
    }
}
