use crate::interfaces::*;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_placement() {
        const N: usize = 10;
        const K: usize = 3;
        const ID: usize = 1;
        let mut player = GuessingPlayer::<N, K> { id: ID };
        let board = Board {
            board: [[Some(0); N]; N],
        };
        let placement = player.do_move(&board);
        let values: Vec<Option<f32>> = placement
            .into_iter()
            .flat_map(|row| row.into_iter())
            .map(|cur| Some(cur))
            .collect();
        let mut old_value = None;
        for val in values {
            if old_value.is_none() {
                old_value = val;
            }
            if old_value != val {
                assert!(false, "Placements not unique!");
            }
        }
        assert!(true);
    }
}
