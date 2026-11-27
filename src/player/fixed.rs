use crate::interfaces::{GameState, Placement, Player, PlayerID};

/// A player that ignores the board and always returns the same placement weights.
///
/// Useful as a deterministic opponent, e.g. for scripting arena scenarios or
/// as a rollout policy whose behaviour is fully known.
pub struct FixedPlayer {
    pub id: PlayerID,
    pub placements: Placement,
}

impl FixedPlayer {
    #[allow(dead_code)]
    #[must_use]
    pub fn new(id: PlayerID, placements: Placement) -> Self {
        Self { id, placements }
    }
}

impl Player for FixedPlayer {
    fn do_move(&mut self, _board: &GameState) -> Placement {
        self.placements.clone()
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
    #[case(0, 3, 3)]
    #[case(1, 2, 5)]
    fn returns_configured_placements_regardless_of_board(
        #[case] id: PlayerID,
        #[case] nrows: u16,
        #[case] ncolumns: u16,
    ) {
        let placements = Placement::new(nrows, ncolumns, 0.5_f32);
        let mut player = FixedPlayer::new(id, placements.clone());
        let empty_board = GameState::new(nrows, ncolumns, None);
        let filled_board = GameState::new(nrows, ncolumns, Some(7));

        assert_eq!(player.get_id(), id);
        assert_eq!(player.do_move(&filled_board), placements);
        assert_eq!(player.do_move(&empty_board), placements);
    }
}
