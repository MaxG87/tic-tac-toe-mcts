MCTS AI for Tic Tac Toe

This project exists so I have a playground to explore two concepts I want to
get more familiar with. The one concept is doing non-trivial programming work
in [Rust](https://www.rust-lang.org/). The other concept is to explore [Monte
Carlo Tree Search](https://en.wikipedia.org/wiki/Monte_Carlo_tree_search).

## CLI

Run a game by choosing player 0, `vs`, and player 1. Player 0 moves first.
The default board is 7×7 with four marks needed to win.

```sh
cargo run -- --size 3 --win-length 3 cli vs minmax --depth 3
cargo run -- --size 3 --win-length 3 mcts --samples 100 --rollout one-lookahead vs guessing
cargo run -- --size 3 --win-length 3 minmax --depth 2 vs minmax --depth 4
```

Available players are `cli` (human), `guessing`, `one-lookahead`, `minmax`,
and `mcts`. The human player reads zero-based `row,column` coordinates from
standard input, for example `0,2`. Invalid or occupied cells are rejected.
The main game uses the exploiting arena: it picks the first highest-weight
legal move, so `guessing` is deterministic in the main game. MCTS simulations
instead use the exploring arena, which samples moves according to their weights.

`minmax` accepts `--depth` (default 4); `mcts` accepts `--samples` (default
1000) and `--rollout guessing|one-lookahead` (default `guessing`). The same
rollout policy is used for both sides in a simulation. Depth, sample count,
board size, and win length must be positive, and win length cannot exceed
board size. Search costs grow quickly with board size and depth.

This CLI uses clap's **nested subcommands** rather than a shared bag of flags.
Each player command owns its arguments, and the first player's `vs` subcommand
contains the second player's commands. Consequently clap rejects, for example,
`cli --depth 3 vs guessing` before any player is constructed. See
`cargo run -- --help`, `cargo run -- minmax --help`, or
`cargo run -- cli vs mcts --help` for context-specific help.
