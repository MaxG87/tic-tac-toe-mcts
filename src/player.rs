use std::iter::Zip;

use crate::arena::*;

struct GuessingPlayer<const N: usize> {
    PLACEMENT: Placement<N>,
}

impl<const N: usize> GuessingPlayer<N> {
    const PLACEMENT: Placement<N> = [[(1.0 / (N as f32)); N]; N];
}

impl<const N: usize> Player<N> for GuessingPlayer<N> {
    fn do_move(&mut self, board: &Board<N>) -> &Placement<N> {
        return &self.PLACEMENT;
    }
}
