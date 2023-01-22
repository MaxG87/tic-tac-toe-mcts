mod arena;
mod interfaces;
mod player;
mod referee;

use crate::arena::exploiting::*;
use crate::interfaces::*;
use crate::player::cli::*;
// use crate::player::countboundmcts::*;
use crate::player::minmax::*;
// use crate::player::onelookahead::*;
use crate::referee::*;

fn main() {
    const N: usize = 5;
    const K: usize = 4;
    let mut mcts_referee = NaiveReferee::<N, K> {};
    // let mut mcts_base_player0 = OneLookaheadPlayer::new(1, Box::new(NaiveReferee::<N, K> {}), 0);
    // let mut mcts_base_player1 = OneLookaheadPlayer::new(0, Box::new(NaiveReferee::<N, K> {}), 1);
    // let mut player0 = CountBoundMCTSPlayer::<N, K>::new(
    //     0,
    //     (N * N * 10000) as u32,
    //     &mut mcts_base_player0,
    //     &mut mcts_base_player1,
    //     &mut mcts_referee,
    // );
    let mut player0 = MinMaxPlayer::<N, K, 0>::new(&mut mcts_referee, 0, 1);
    let mut player1 = CLIPlayer::<N, K> { id: 1 };
    let mut referee = NaiveReferee::<N, K> {};
    let board = Board {
        board: [[None; N]; N],
    };
    let mut arena =
        ExploitingArena::<N, K>::new(1, board, [&mut player1, &mut player0], &mut referee);
    loop {
        let (maybe_result, player_id, maybe_point_placement) = arena.do_next_move();
        println!(
            "Player {player_id} made {}.",
            match maybe_point_placement {
                Some(pp) => format!("move {}", pp),
                None => "no legal move".to_string(),
            }
        );
        let board = arena.get_board();
        println!("{board}");
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
