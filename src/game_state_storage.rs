use crate::board::Board;
use std::collections::HashMap;

pub trait GameStateStorage<Payload, DepthT: std::cmp::PartialOrd + Copy = u32> {
    fn register_game_state(&mut self, board: &Board, payload: Payload, depth: DepthT);
    fn get_payload(&self, board: &Board, depth: DepthT) -> Option<&Payload>;
}

pub struct NaiveGameStateStorage<Payload, DepthT: std::cmp::PartialOrd + Copy = u32> {
    storage: HashMap<Board, (DepthT, Payload)>,
}

impl<Payload, DepthT: std::cmp::PartialOrd + Copy>
    NaiveGameStateStorage<Payload, DepthT>
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl<Payload, DepthT: std::cmp::PartialOrd + Copy> GameStateStorage<Payload, DepthT>
    for NaiveGameStateStorage<Payload, DepthT>
{
    /// Registers a game state with a given payload and depth.
    ///
    /// # Arguments
    /// * `board` - The board to register.
    /// * `payload` - The payload for the given board.
    /// * `depth` - The search depth used to derive the payload.
    fn register_game_state(&mut self, board: &Board, payload: Payload, depth: DepthT) {
        if self.get_payload(board, depth).is_none() {
            self.storage.insert(board.clone(), (depth, payload));
        }
    }
    /// Retrieves the payload for a given board.
    ///
    /// Given a board and a search depth, this function will return the associated payload, if
    /// there is one that was derived with a lookahead of at least the given depth.
    ///
    /// # Arguments
    /// * `board` - The board to retrieve the payload for.
    /// * `depth` - The minimal required search depth.
    fn get_payload(&self, board: &Board, depth: DepthT) -> Option<&Payload> {
        self.storage.get(board).and_then(|(stored_depth, payload)| {
            if *stored_depth >= depth {
                Some(payload)
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interfaces::PointPlacement;

    #[test]
    fn test_can_store_and_retrieve() {
        const N: usize = 3;
        let payload = "Some Payload!".to_string();
        let depth = 2;
        let board = Board::new(u16::try_from(N).unwrap(), u16::try_from(N).unwrap());
        let mut storage = NaiveGameStateStorage::<String>::new();

        storage.register_game_state(&board, payload.clone(), depth);
        let result = storage.get_payload(&board, depth);
        assert_eq!(result, Some(&payload));
    }

    #[test]
    fn test_can_retrieve_states_with_bigger_lookahead() {
        /// This test tests wether the storage can retrieve states with a bigger lookahead than the
        /// one it was registered with.
        const N: usize = 3;
        let payload = "Some Payload!".to_string();
        let depth = 2;
        let board = Board::new(u16::try_from(N).unwrap(), u16::try_from(N).unwrap());
        let mut storage = NaiveGameStateStorage::<String>::new();

        storage.register_game_state(&board, payload.clone(), depth + 1);
        let result = storage.get_payload(&board, depth);
        assert_eq!(result, Some(&payload));
    }

    #[test]
    fn test_nothing_retrived_for_wrong_payload() {
        /// This test tests wether the storage can retrieve states with a bigger lookahead than the
        /// one it was registered with.
        const N: usize = 3;
        let payload = "Some Payload!".to_string();
        let depth = 2;
        let board = Board::new(u16::try_from(N).unwrap(), u16::try_from(N).unwrap());
        let mut new_board =
            Board::new(u16::try_from(N).unwrap(), u16::try_from(N).unwrap());
        let mut storage = NaiveGameStateStorage::<String>::new();

        new_board[PointPlacement { row: 0, column: 0 }] = Some(1);
        storage.register_game_state(&board, payload.clone(), depth + 1);
        let result = storage.get_payload(&new_board, depth);
        assert_eq!(result, None);
    }

    #[test]
    fn test_nothing_retrived_for_to_deep_depth() {
        /// This test tests wether the storage can retrieve states with a bigger lookahead than the
        /// one it was registered with.
        const N: usize = 7;
        let payload = "Some Payload!".to_string();
        let depth = 5;
        let board = Board::new(u16::try_from(N).unwrap(), u16::try_from(N).unwrap());
        let mut storage = NaiveGameStateStorage::<String>::new();

        storage.register_game_state(&board, payload.clone(), depth - 1);
        let result = storage.get_payload(&board, depth);
        assert_eq!(result, None);
    }

    #[test]
    fn test_registering_deeper_boards_overwrites_shallower_ones() {
        const N: usize = 7;
        let deep_payload = "Deep Payload!".to_string();
        let shallow_payload = "Shallow Payload!".to_string();
        let depth = 5;
        let board = Board::new(u16::try_from(N).unwrap(), u16::try_from(N).unwrap());
        let mut storage = NaiveGameStateStorage::<String>::new();

        storage.register_game_state(&board, shallow_payload.clone(), depth - 1);
        storage.register_game_state(&board, deep_payload.clone(), depth);
        let shallow_result = storage.get_payload(&board, depth - 1);
        let deep_result = storage.get_payload(&board, depth);
        assert_eq!(shallow_result, Some(&deep_payload));
        assert_eq!(deep_result, Some(&deep_payload));
    }

    #[test]
    fn test_registering_shallow_board_does_not_overwrite_deeper_one() {
        const N: usize = 7;
        let deep_payload = "Deep Payload!".to_string();
        let shallow_payload = "Shallow Payload!".to_string();
        let depth = 5;
        let board = Board::new(u16::try_from(N).unwrap(), u16::try_from(N).unwrap());
        let mut storage = NaiveGameStateStorage::<String>::new();

        storage.register_game_state(&board, deep_payload.clone(), depth);
        storage.register_game_state(&board, shallow_payload.clone(), depth - 1);
        let result = storage.get_payload(&board, depth - 1);
        assert_eq!(result, Some(&deep_payload));
    }
}
