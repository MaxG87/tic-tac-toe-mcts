use crate::interfaces::{
    BoardSizeT, GameState, Placement, Player, PlayerID, PointPlacement, WinLengthT,
};
use std::io;

pub struct CLIPlayer<const N: BoardSizeT, const K: WinLengthT> {
    pub id: PlayerID,
}

impl<const N: BoardSizeT, const K: WinLengthT> CLIPlayer<N, K> {
    fn get_point_placement(&self) -> PointPlacement {
        let _ = self; // self is not needed here.
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let line = buffer.trim();
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 2 {
                continue;
            }
            let row = parts[0].parse::<BoardSizeT>();
            let column = parts[1].parse::<BoardSizeT>();
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

impl<const N: BoardSizeT, const K: WinLengthT> Player<N, K> for CLIPlayer<N, K> {
    fn do_move(&mut self, _: &GameState) -> Placement<N> {
        let point_placement = self.get_point_placement();
        let mut placements: Placement<N> = [[0.0; N]; N];
        placements[point_placement.row][point_placement.column] = 1.0;
        placements
    }
    fn get_id(&self) -> PlayerID {
        self.id
    }
}
