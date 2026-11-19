use crate::interfaces::{
    BoardSizeT, GameState, Placement, Player, PlayerID, PointPlacement,
};
use std::io::{self, Write};

#[allow(dead_code)]
pub struct CLIPlayer {
    pub id: PlayerID,
}

#[allow(dead_code)]
impl CLIPlayer {
    fn get_point_placement(&self, board: &GameState) -> PointPlacement {
        let nrows = board.get_number_of_rows();
        let ncolumns = board.get_number_of_columns();
        loop {
            print!("Player {}: enter row,column (zero-based): ", self.id);
            if let Err(error) = io::stdout().flush() {
                eprintln!("Failed to write prompt: {error}");
                std::process::exit(1);
            }
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(0) => {
                    eprintln!("Unexpected end of input while waiting for a move.");
                    std::process::exit(1);
                }
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Failed to read move: {error}");
                    std::process::exit(1);
                }
            }
            let line = buffer.trim();
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 2 {
                eprintln!("Enter a move as row,column.");
                continue;
            }
            let row = parts[0].trim().parse::<BoardSizeT>();
            let column = parts[1].trim().parse::<BoardSizeT>();
            let point_placement = if let (Ok(row), Ok(column)) = (row, column) {
                PointPlacement { row, column }
            } else {
                eprintln!("Row and column must be non-negative integers.");
                continue;
            };
            if point_placement.row >= nrows || point_placement.column >= ncolumns {
                eprintln!("That move is outside the board.");
            } else if board[point_placement].is_taken() {
                eprintln!("That cell is already occupied.");
            } else {
                return point_placement;
            }
        }
    }
}

impl Player for CLIPlayer {
    fn do_move(&mut self, board: &GameState) -> Placement {
        let point_placement = self.get_point_placement(board);
        let mut placements = Placement::new_from_existing(board, 0.0_f32);
        placements[point_placement] = 1.0;
        placements
    }
    fn get_id(&self) -> PlayerID {
        self.id
    }
}
