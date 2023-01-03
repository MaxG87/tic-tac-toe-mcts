use std::iter::Zip;

use crate::arena;

struct GuessingPlayer<const N: usize> {}

impl<const N: usize> arena::Player<N> for GuessingPlayer<N> {
    fn do_move(&mut self, board: &arena::Board<N>) -> arena::Placement {
        // let free_slots = Zip{0..board.rows(), 0..board.cols()}
        unimplemented!()
    }
}
