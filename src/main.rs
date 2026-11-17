use crate::game_state_storage::NaiveGameStateStorage;
use arena::exploiting::ExploitingArena;
use command_line::{PlayerConfig, Rollout};
use interfaces::{GameResult, GameState, Player, PlayerID, TicTacToeArena, WinLengthT};
use player::{
    cli::CLIPlayer, countboundmcts::CountBoundMCTSPlayer, guessing::GuessingPlayer,
    minmax::MinMaxPlayer, onelookahead::OneLookaheadPlayer,
};
use referee::NaiveReferee;

mod arena;
mod board;
mod command_line;
mod game_state_storage;
mod interfaces;
mod player;
mod referee;

struct MctsResources {
    players: [Box<dyn Player>; 2],
    referee: NaiveReferee,
}

impl MctsResources {
    fn new(rollout: Rollout, win_length: WinLengthT) -> Self {
        let player = |id| -> Box<dyn Player> {
            match rollout {
                Rollout::Guessing => Box::new(GuessingPlayer { id }),
                Rollout::OneLookahead => Box::new(OneLookaheadPlayer::new(
                    1 - id,
                    Box::new(NaiveReferee::new(win_length)),
                    id,
                )),
            }
        };
        Self {
            players: [player(0), player(1)],
            referee: NaiveReferee::new(win_length),
        }
    }
}

fn build_player<'a>(
    config: PlayerConfig,
    id: PlayerID,
    win_length: WinLengthT,
    referee: &'a NaiveReferee,
    resources: &'a mut MctsResources,
) -> Box<dyn Player + 'a> {
    match config {
        PlayerConfig::Cli => Box::new(CLIPlayer { id }),
        PlayerConfig::Guessing => Box::new(GuessingPlayer { id }),
        PlayerConfig::OneLookahead => Box::new(OneLookaheadPlayer::new(
            1 - id,
            Box::new(NaiveReferee::new(win_length)),
            id,
        )),
        PlayerConfig::MinMax { depth } => Box::new(MinMaxPlayer::new(
            depth,
            1 - id,
            NaiveGameStateStorage::new(),
            referee,
            id,
        )),
        PlayerConfig::Mcts { samples, .. } => {
            let [player0, player1] = &mut resources.players;
            Box::new(CountBoundMCTSPlayer::new(
                id,
                samples,
                &mut **player0,
                &mut **player1,
                &mut resources.referee,
            ))
        }
    }
}

fn main() {
    let config = command_line::parse();
    let search_referee = NaiveReferee::new(config.win_length);
    let mut referee = NaiveReferee::new(config.win_length);
    let resources = |player| {
        let rollout = match player {
            PlayerConfig::Mcts { rollout, .. } => rollout,
            _ => Rollout::Guessing,
        };
        MctsResources::new(rollout, config.win_length)
    };
    let mut resources0 = resources(config.players[0]);
    let mut resources1 = resources(config.players[1]);
    let mut player0 = build_player(
        config.players[0],
        0,
        config.win_length,
        &search_referee,
        &mut resources0,
    );
    let mut player1 = build_player(
        config.players[1],
        1,
        config.win_length,
        &search_referee,
        &mut resources1,
    );
    let board = GameState::new(config.size, config.size, None);
    let mut arena =
        ExploitingArena::new(0, board, [&mut *player0, &mut *player1], &mut referee);
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
            GameResult::Defeat => Some(format!("Player {player_id} lost.")),
            GameResult::Victory => Some(format!("Player {player_id} won.")),
            GameResult::Draw => Some("The game ended draw!".to_string()),
            GameResult::IllegalMove => {
                Some(format!("Player {player_id} made an illegal move."))
            }
            GameResult::Undecided => None,
        };
        if let Some(val) = maybe_result_msg {
            println!("{val}");
            break;
        }
    }
}
