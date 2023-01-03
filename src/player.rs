use crate::arena::*;
use std::io;

pub struct GuessingPlayer<const N: usize, const K: usize> {
    pub id: PlayerID,
}

impl<const N: usize, const K: usize> GuessingPlayer<N, K> {
    const PLACEMENT: Placement<N> = [[1.0; N]; N];
}

impl<const N: usize, const K: usize> Player<N, K> for GuessingPlayer<N, K> {
    fn do_move(&mut self, _: &Board<N>) -> Placement<N> {
        return GuessingPlayer::<N, K>::PLACEMENT.clone();
    }

    fn get_id(&self) -> PlayerID {
        return self.id;
    }
}

pub struct CLIPlayer<const N: usize, const K: usize> {
    pub id: PlayerID,
}
impl<const N: usize, const K: usize> CLIPlayer<N, K> {
    fn get_point_placement(&self) -> PointPlacement {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let line = buffer.trim();
            let parts: Vec<&str> = line.split(",").collect();
            if parts.len() != 2 {
                continue;
            }
            let row = parts[0].parse::<usize>();
            let column = parts[1].parse::<usize>();
            let point_placement = match (row, column) {
                (Ok(row), Ok(column)) => PointPlacement { row, column },
                _ => continue,
            };
            if point_placement.row < N && point_placement.column < N {
                return point_placement;
            }
        }
    }
}
impl<const N: usize, const K: usize> Player<N, K> for CLIPlayer<N, K> {
    fn do_move(&mut self, _: &Board<N>) -> Placement<N> {
        let point_placement = self.get_point_placement();
        let mut placements: Placement<N> = [[0.0; N]; N];
        placements[point_placement.row][point_placement.column] = 1.0;
        return placements;
    }
    fn get_id(&self) -> PlayerID {
        return self.id;
    }
}

pub struct CountBoundMCTSPlayer<'player, const N: usize, const K: usize> {
    id: PlayerID,
    nsamples: u32,
    referee: &'player mut dyn TicTacToeReferee<N, K>,
    player0: &'player mut dyn Player<N, K>,
    player1: &'player mut dyn Player<N, K>,
}
impl<'player, const N: usize, const K: usize> CountBoundMCTSPlayer<'player, N, K> {
    pub fn new(
        id: PlayerID,
        nsamples: u32,
        player0: &'player mut dyn Player<N, K>,
        player1: &'player mut dyn Player<N, K>,
        referee: &'player mut dyn TicTacToeReferee<N, K>,
    ) -> CountBoundMCTSPlayer<'player, N, K> {
        CountBoundMCTSPlayer {
            id,
            nsamples,
            player0,
            player1,
            referee,
        }
    }
}
impl<'player, const N: usize, const K: usize> Player<N, K> for CountBoundMCTSPlayer<'player, N, K> {
    fn do_move(&mut self, board: &Board<N>) -> Placement<N> {
        let mut tries = [[0u32; N]; N];
        let mut wins = [[0u32; N]; N];

        for _ in 0..self.nsamples {
            let mut my_arena = TicTacToeArena::<N, K>::new(
                board.clone(),
                [&mut *self.player0, &mut *self.player1],
                &mut *self.referee,
            );

            let (result, player_id, first_point_placement) =
                CountBoundMCTSPlayer::do_one_step_sample(&mut my_arena);

            match first_point_placement {
                Some(pp) => {
                    tries[pp.row][pp.column] += 1;
                    if result == Result::Victory && self.id == player_id {
                        wins[pp.row][pp.column] += 1;
                    }
                }
                None => panic!("No legal move was made!"),
            }
        }
        let mut placements: Placement<N> = [[0f32; N]; N];
        for row in 0..N {
            for column in 0..N {
                placements[row][column] = if tries[row][column] == 0 {
                    0.0
                } else {
                    (wins[row][column] as f32) / (tries[row][column] as f32)
                };
            }
        }

        println!("{:?}", placements);
        return placements;
    }

    fn get_id(&self) -> PlayerID {
        return self.id;
    }
}

impl<'player, const N: usize, const K: usize> CountBoundMCTSPlayer<'player, N, K> {
    fn do_one_step_sample(
        arena: &mut TicTacToeArena<N, K>,
    ) -> (Result, PlayerID, Option<PointPlacement>) {
        let (first_result, first_player_id, first_point_placement) = arena.do_next_move();
        if let Some(result) = first_result {
            return (result, first_player_id, first_point_placement);
        }
        loop {
            match arena.do_next_move() {
                (Some(result), player_id, _) => return (result, player_id, first_point_placement),
                (None, _, _) => continue,
            }
        }
    }
}
