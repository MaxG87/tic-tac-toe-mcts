// use crate::player::countboundmcts::*;
// use crate::player::onelookahead::*;
use crate::game_state_storage::*;
use arena::exploiting::*;
use interfaces::{Board, BoardSizeT, Evaluation, Result, TicTacToeArena, WinLengthT};
use player::cli::*;
use player::minmax::*;
use referee::*;

mod arena;
mod game_state_storage;
mod interfaces;
mod player;
mod referee;
mod utils;

fn main() {
    const N: BoardSizeT = 7;
    const K: WinLengthT = 3;
    let mut mcts_referee = NaiveReferee::<N, K> {};
    let mut game_state_storage = NaiveGameStateStorage::<N, Evaluation<N>>::new();

    // let mut mcts_base_player0 = OneLookaheadPlayer::new(1, Box::new(NaiveReferee::<N, K> {}), 0);
    // let mut mcts_base_player1 = OneLookaheadPlayer::new(0, Box::new(NaiveReferee::<N, K> {}), 1);
    // let mut player0 = CountBoundMCTSPlayer::<N, K>::new(
    //     0,
    //     (N * N * 10000) as WinLengthT,
    //     &mut mcts_base_player0,
    //     &mut mcts_base_player1,
    //     &mut mcts_referee,
    // );
    let mut player0 =
        MinMaxPlayer::<N, K>::new(4, 1, &mut game_state_storage, &mut mcts_referee, 0);
    let mut player1 = CLIPlayer::<N, K> { id: 1 };
    let mut referee = NaiveReferee::<N, K> {};
    let board = Board::<N>::new();
    let mut arena = ExploitingArena::<N, K>::new(
        0,
        board,
        [&mut player1, &mut player0],
        &mut referee,
    );
    loop {
        let (result, player_id, maybe_point_placement) = arena.do_next_move();
        println!(
            "Player {player_id} made {}.",
            match maybe_point_placement {
                Some(pp) => format!("move {pp}"),
                None => "no legal move".to_string(),
            }
        );
        let board = arena.get_board();
        println!("{board}");
        let maybe_result_msg: Option<String> = match result {
            Result::Defeat => Some(format!("Player {player_id} lost.")),
            Result::Victory => Some(format!("Player {player_id} won.")),
            Result::Draw => Some("The game ended draw!".to_string()),
            Result::IllegalMove => {
                Some(format!("Player {player_id} made an illegal move."))
            }
            Result::Undecided => None,
        };
        if maybe_result_msg.is_some() {
            println!("{}", maybe_result_msg.unwrap());
            break;
        }
    }
}
