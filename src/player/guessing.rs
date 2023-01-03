use crate::arena::*;

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
