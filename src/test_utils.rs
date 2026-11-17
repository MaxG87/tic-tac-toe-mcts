//! Helpers that keep parametrised test cases short and readable.

use crate::interfaces::{GameState, Placement, PlayerID, PointPlacement};

pub fn board(rows: Vec<Vec<Option<PlayerID>>>) -> GameState {
    GameState::new_with_values(rows).unwrap()
}

pub fn placement(rows: Vec<Vec<f32>>) -> Placement {
    Placement::new_with_values(rows).unwrap()
}

pub fn pp(row: u16, column: u16) -> PointPlacement {
    PointPlacement { row, column }
}
