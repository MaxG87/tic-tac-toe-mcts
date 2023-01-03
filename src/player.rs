use crate::arena::*;

pub struct GuessingPlayer<const N: usize> {
    pub id: PlayerID,
}

impl<const N: usize> GuessingPlayer<N> {
    const PLACEMENT: Placement<N> = [[(1.0 / ((N * N) as f32)); N]; N];
}

impl<const N: usize> Player<N> for GuessingPlayer<N> {
    fn do_move(&mut self, _board: &Board<N>) -> &Placement<N> {
        return &GuessingPlayer::<N>::PLACEMENT;
    }

    fn get_id(&self) -> PlayerID {
        return self.id;
    }
}
