use clap::{Arg, ArgMatches, Command, error::ErrorKind, value_parser};

use crate::interfaces::{BoardSizeT, WinLengthT};

type MinMaxDepthT = u8;
type MctsNumSamplesT = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rollout {
    Guessing,
    OneLookahead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerConfig {
    Cli,
    Guessing,
    OneLookahead,
    MinMax { depth: MinMaxDepthT },
    Mcts { samples: MctsNumSamplesT, rollout: Rollout },
}

pub struct Config {
    pub size: BoardSizeT,
    pub win_length: WinLengthT,
    pub players: [PlayerConfig; 2],
}

fn player_commands(first: bool) -> Vec<Command> {
    let players = [
        Command::new("cli").about("A human entering moves on stdin"),
        Command::new("guessing").about("Assign equal weight to all moves"),
        Command::new("one-lookahead").about("Look for immediate wins and threats"),
        Command::new("minmax")
            .about("Search possible moves with minimax")
            .arg(
                Arg::new("depth")
                    .long("depth")
                    .help("Search depth (at least 1)")
                    .value_parser(value_parser!(MinMaxDepthT).range(1..))
                    .default_value("4"),
            ),
        Command::new("mcts")
            .about("Sample games using count-bound Monte Carlo search")
            .arg(
                Arg::new("samples")
                    .long("samples")
                    .help("Number of simulated games per move")
                    .value_parser(value_parser!(MctsNumSamplesT).range(1..))
                    .default_value("1000"),
            )
            .arg(
                Arg::new("rollout")
                    .long("rollout")
                    .help("Player used for both sides in simulations")
                    .value_parser(["guessing", "one-lookahead"])
                    .default_value("guessing"),
            ),
    ];

    players
        .into_iter()
        .map(|player| {
            if first {
                player
                    .subcommand(
                        Command::new("vs")
                            .about("Choose player 1 (player 0 moves first)")
                            .subcommand_required(true)
                            .arg_required_else_help(true)
                            .subcommands(player_commands(false)),
                    )
                    .subcommand_required(true)
                    .arg_required_else_help(true)
            } else {
                player
            }
        })
        .collect()
}

fn command() -> Command {
    Command::new("tic-tac-toe-mcts")
        .about("Play a game between two configurable players")
        .after_help("Choose player 0, then 'vs', then player 1. For example:\n  tic-tac-toe-mcts --size 3 --win-length 3 minmax --depth 2 vs cli\n\nUse '<player> --help' or '<player> vs <player> --help' for player-specific options.")
        .arg(
            Arg::new("size")
                .long("size")
                .help("Side length of the square board")
                .value_parser(value_parser!(u16).range(1..))
                .default_value("7"),
        )
        .arg(
            Arg::new("win-length")
                .long("win-length")
                .help("Number of consecutive marks needed to win")
                .value_parser(value_parser!(u16).range(1..))
                .default_value("4"),
        )
        .subcommands(player_commands(true))
        .subcommand_required(true)
        .arg_required_else_help(true)
}

fn player_config(name: &str, args: &ArgMatches) -> PlayerConfig {
    match name {
        "cli" => PlayerConfig::Cli,
        "guessing" => PlayerConfig::Guessing,
        "one-lookahead" => PlayerConfig::OneLookahead,
        "minmax" => PlayerConfig::MinMax {
            depth: *args.get_one("depth").expect("depth has a default"),
        },
        "mcts" => PlayerConfig::Mcts {
            samples: *args
                .get_one("samples")
                .expect("samples has a default"),
            rollout: match args.get_one::<String>("rollout").map(String::as_str) {
                Some("guessing") => Rollout::Guessing,
                Some("one-lookahead") => Rollout::OneLookahead,
                _ => unreachable!("clap validates rollout"),
            },
        },
        _ => unreachable!("clap validates player commands"),
    }
}

fn config(matches: &ArgMatches) -> Result<Config, clap::Error> {
    let size = *matches.get_one("size").expect("size has a default");
    let win_length = *matches
        .get_one("win-length")
        .expect("win-length has a default");
    if win_length > size {
        return Err(command().error(
            ErrorKind::ValueValidation,
            "--win-length must not exceed --size",
        ));
    }

    let (first_name, first_args) = matches.subcommand().expect("player 0 is required");
    let (_, vs_args) = first_args.subcommand().expect("vs is required");
    let (second_name, second_args) =
        vs_args.subcommand().expect("player 1 is required");
    Ok(Config {
        size,
        win_length,
        players: [
            player_config(first_name, first_args),
            player_config(second_name, second_args),
        ],
    })
}

pub fn parse() -> Config {
    let matches = command().get_matches();
    config(&matches).unwrap_or_else(|error| error.exit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independently_parses_player_options() {
        let matches = command()
            .try_get_matches_from([
                "game",
                "minmax",
                "--depth",
                "2",
                "vs",
                "mcts",
                "--samples",
                "12",
                "--rollout",
                "one-lookahead",
            ])
            .unwrap();
        let config = config(&matches).unwrap();
        assert_eq!(
            config.players,
            [
                PlayerConfig::MinMax { depth: 2 },
                PlayerConfig::Mcts {
                    samples: 12,
                    rollout: Rollout::OneLookahead,
                },
            ]
        );
    }

    #[test]
    fn rejects_options_for_the_wrong_player_and_invalid_values() {
        for args in [
            vec!["game", "cli", "--depth", "2", "vs", "guessing"],
            vec!["game", "minmax", "vs", "cli", "--depth", "2"],
            vec!["game", "minmax", "--depth", "0", "vs", "cli"],
            vec!["game", "mcts", "--samples", "0", "vs", "cli"],
            vec!["game", "--size", "0", "cli", "vs", "cli"],
            vec!["game", "cli", "vs"],
        ] {
            assert!(
                command().try_get_matches_from(args.clone()).is_err(),
                "{args:?}"
            );
        }
    }

    #[test]
    fn rejects_win_length_larger_than_board() {
        let matches = command()
            .try_get_matches_from(["game", "--size", "3", "cli", "vs", "cli"])
            .unwrap();
        assert!(config(&matches).is_err());
    }
}
