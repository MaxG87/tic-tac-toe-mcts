use crate::interfaces::{GameState, Placement, Player, PlayerID};

#[allow(dead_code)]
pub struct GuessingPlayer {
    pub id: PlayerID,
}

impl GuessingPlayer {}

impl Player for GuessingPlayer {
    fn do_move(&mut self, board: &GameState) -> Placement {
        Placement::new_from_existing(board, 1.0_f32)
    }

    fn get_id(&self) -> PlayerID {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 1, Some(0))]
    #[case(0, 3, 3, None)]
    #[case(1, 10, 10, None)]
    #[case(5, 2, 7, Some(1))]
    fn assigns_one_to_every_cell_and_reports_id(
        #[case] id: PlayerID,
        #[case] nrows: u16,
        #[case] ncolumns: u16,
        #[case] occupant: Option<PlayerID>,
    ) {
        let mut player = GuessingPlayer { id };
        let board = GameState::new(nrows, ncolumns, occupant);

        let placement = player.do_move(&board);

        assert_eq!(player.get_id(), id);
        assert_eq!(placement, Placement::new(nrows, ncolumns, 1.0_f32));
    }
}
