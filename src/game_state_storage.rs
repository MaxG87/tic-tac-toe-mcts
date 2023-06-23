use crate::interfaces::*;
use std::collections::*;

pub trait GameStateStorage<const N: usize, Payload> {
    fn register_game_state(&mut self, board: &Board<N>, payload: Payload, depth: usize);
    fn get_payload(&self, board: &Board<N>, depth: usize) -> Option<&Payload>;
}

struct NaiveGameStateStorage<const N: usize, Payload> {
    storage: HashMap<(Board<N>, usize), Payload>,
}

impl<const N: usize, Payload> NaiveGameStateStorage<N, Payload> {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl<const N: usize, Payload> GameStateStorage<N, Payload> for NaiveGameStateStorage<N, Payload> {
    fn register_game_state(&mut self, board: &Board<N>, payload: Payload, depth: usize) {
        self.storage.insert((board.clone(), depth), payload);
    }
    fn get_payload(&self, board: &Board<N>, depth: usize) -> Option<&Payload> {
        self.storage.get(&(board.clone(), depth))
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
        let result: Option<&String> = storage.get_payload(&board, depth);
        assert_eq!(result, Some(&payload));
    }
}
