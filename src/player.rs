use crate::arena;

struct GuessingPlayer<const N: u32> {}

impl<const N: u32> arena::Player<N> for GuessingPlayer<N> {
    fn do_move(&mut self, board: &arena::BoardState) -> arena::Placement {
        unimplemented!();
    }
}
