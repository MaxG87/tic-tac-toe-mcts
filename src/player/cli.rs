use crate::interfaces::{
    BoardSizeT, GameState, Placement, Player, PlayerID, PointPlacement, WinLengthT,
};
use std::io;

pub struct CLIPlayer<const K: WinLengthT> {
    pub id: PlayerID,
}

impl<const K: WinLengthT> CLIPlayer<K> {
    fn get_point_placement(&self, board: &GameState) -> PointPlacement {
        let _ = self; // self is not needed here.
        let nrows = board.get_number_of_rows().into();
        let ncolumns = board.get_number_of_columns().into();
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
            if point_placement.row < nrows && point_placement.column < ncolumns {
                return point_placement;
            }
        }
    }
}

impl<const K: WinLengthT> Player<K> for CLIPlayer<K> {
    fn do_move(&mut self, board: &GameState) -> Placement {
        let point_placement = self.get_point_placement(board);
        let mut placements = Placement::new_from_existing(board, 0.0);
        placements[point_placement] = 1.0;
        placements
    }
    fn get_id(&self) -> PlayerID {
        self.id
    }
}
