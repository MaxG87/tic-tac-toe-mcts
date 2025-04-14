use crate::interfaces::*;
use std::collections::*;

pub trait GameStateStorage<const N: usize, Payload> {
    fn register_game_state(&mut self, board: Board<N>, payload: Payload, depth: usize);
    fn get_payload(self, board: Board<N>, depth: usize) -> Option<Payload>;
}

struct NaiveGameStateStorage<const N: usize, Payload> {
    storage: HashMap<(Board<N>, usize), Payload>,
}

impl<const N: usize, Payload> NaiveGameStateStorage<N, Payload> {
    pub fn new() -> Self {
        Self {
            storage: HashMap::<(Board<N>, usize), Payload>::new(),
        }
    }
}

impl<const N: usize, Payload> GameStateStorage<N, Payload> for NaiveGameStateStorage<N, Payload> {
    fn register_game_state(&mut self, _board: Board<N>, _payload: Payload, _depth: usize) {}
    fn get_payload(self, _board: Board<N>, _depth: usize) -> Option<Payload> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initial_test() {
        const N: usize = 3;
        let board = Board::<N>::new();
        let storage = NaiveGameStateStorage::<N, usize>::new();
        assert_eq!(storage.get_payload(board, 0), None);
    }
}
