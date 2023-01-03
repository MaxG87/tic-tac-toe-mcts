mod arena;
mod player;
mod referee;

use crate::arena::*;
use crate::player::*;
use crate::referee::*;

fn main() {
    const N: usize = 5;
    let players: [Box<dyn Player<N>>; 2] = [
        Box::new(GuessingPlayer::<N> { id: 0 }),
        Box::new(GuessingPlayer::<N> { id: 1 }),
    ];
    let referee = Box::new(NaiveReferee::<N> {});
    let board = Board {
        board: [[None; N]; N],
    };
    let mut arena = TicTacToeArena::<N>::new(board, players, referee);
    loop {
        let (maybe_result, player_id) = arena.do_next_move();
        let board = arena.get_board();
        let maybe_result_msg: Option<String> = match maybe_result {
            Some(Result::Defeat) => Some(format!("Player {player_id} lost.")),
            Some(Result::Victory) => Some(format!("Player {player_id} won.")),
            Some(Result::Draw) => Some("The game ended draw!".to_string()),
            Some(Result::IllegalMove) => Some(format!("Player {player_id} made an illegal move.")),
            None => None,
        };
        if let Some(msg) = maybe_result_msg {
            println!("{}", msg);
            break;
        }
    }
}
