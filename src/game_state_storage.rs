use crate::interfaces::*;
use std::collections::*;

pub trait GameStateStorage<const N: usize, Payload, DepthT: std::cmp::PartialOrd = u32>
{
    fn register_game_state(
        &mut self,
        board: &Board<N>,
        payload: Payload,
        depth: DepthT,
    );
    fn get_payload(&self, board: &Board<N>, depth: DepthT) -> Option<&Payload>;
}

pub struct NaiveGameStateStorage<
    const N: usize,
    Payload,
    DepthT: std::cmp::PartialOrd = u32,
> {
    storage: HashMap<Board<N>, (DepthT, Payload)>,
}

impl<const N: usize, Payload, DepthT: std::cmp::PartialOrd>
    NaiveGameStateStorage<N, Payload, DepthT>
{
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl<const N: usize, Payload, DepthT: std::cmp::PartialOrd>
    GameStateStorage<N, Payload, DepthT> for NaiveGameStateStorage<N, Payload, DepthT>
{
    /// Registers a game state with a given payload and depth.
    ///
    /// # Arguments
    /// * `board` - The board to register.
    /// * `payload` - The payload for the given board.
    /// * `depth` - The search depth used to derive the payload.
    fn register_game_state(
        &mut self,
        board: &Board<N>,
        payload: Payload,
        depth: DepthT,
    ) {
        self.storage.insert(board.clone(), (depth, payload));
    }
    /// Retrieves the payload for a given board.
    ///
    /// Given a board and a search depth, this function will return the associated payload, if
    /// there is one that was derived with a lookahead of at least the given depth.
    ///
    /// # Arguments
    /// * `board` - The board to retrieve the payload for.
    /// * `depth` - The minimal required search depth.
    fn get_payload(&self, board: &Board<N>, depth: DepthT) -> Option<&Payload> {
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
    #[test]
    fn test_can_store_and_retrieve() {
        const N: usize = 3;
        let payload = "Some Payload!".to_string();
        let depth = 2;
        let board = Board::<N>::new();
        let mut storage = NaiveGameStateStorage::<N, String>::new();

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
        let board = Board::<N>::new();
        let mut storage = NaiveGameStateStorage::<N, String>::new();

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
        let board = Board::<N>::new();
        let mut new_board = Board::<N>::new();
        let mut storage = NaiveGameStateStorage::<N, String>::new();

        new_board.set_placement_at(PointPlacement { row: 0, column: 0 }, Some(1));
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
        let board = Board::<N>::new();
        let mut storage = NaiveGameStateStorage::<N, String>::new();

        storage.register_game_state(&board, payload.clone(), depth - 1);
        let result = storage.get_payload(&board, depth);
        assert_eq!(result, None);
    }
}
