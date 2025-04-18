// use crate::player::countboundmcts::*;
// use crate::player::onelookahead::*;
use crate::game_state_storage::NaiveGameStateStorage;
use arena::exploiting::ExploitingArena;
use interfaces::{
    BoardSizeT, Evaluation, GameState, Result, TicTacToeArena, WinLengthT,
};
use player::cli::CLIPlayer;
use player::minmax::MinMaxPlayer;
use referee::NaiveReferee;

mod arena;
mod board;
mod game_state_storage;
mod interfaces;
mod player;
mod referee;

fn main() {
    const N: BoardSizeT = 7;
    const K: WinLengthT = 3;
    let mut mcts_referee = NaiveReferee::new(K);
    let mut game_state_storage = NaiveGameStateStorage::<_, Evaluation>::new();

    // let mut mcts_base_player0 = OneLookaheadPlayer::new(1, Box::new(NaiveReferee::<K> {}), 0);
    // let mut mcts_base_player1 = OneLookaheadPlayer::new(0, Box::new(NaiveReferee::<K> {}), 1);
    // let mut player0 = CountBoundMCTSPlayer::<K>::new(
    //     0,
    //     (N * N * 10000) as WinLengthT,
    //     &mut mcts_base_player0,
    //     &mut mcts_base_player1,
    //     &mut mcts_referee,
    // );
    let mut player0 =
        MinMaxPlayer::new(5, 1, &mut game_state_storage, &mut mcts_referee, 0);
    let mut player1 = CLIPlayer { id: 1 };
    let mut referee = NaiveReferee::new(K);
    let board = GameState::new(N, N, None);
    let mut arena =
        ExploitingArena::new(0, board, [&mut player1, &mut player0], &mut referee);
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
