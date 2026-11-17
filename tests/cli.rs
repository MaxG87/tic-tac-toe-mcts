use std::io::Write;
use std::process::{Command, Stdio};

fn game(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tic-tac-toe-mcts"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn configured_players_complete_games() {
    for players in [
        vec!["guessing", "vs", "one-lookahead"],
        vec!["minmax", "--depth", "1", "vs", "minmax", "--depth", "2"],
        vec![
            "mcts",
            "--samples",
            "5",
            "--rollout",
            "one-lookahead",
            "vs",
            "mcts",
            "--samples",
            "5",
        ],
    ] {
        let mut args = vec!["--size", "2", "--win-length", "2"];
        args.extend(players);
        let output = game(&args);
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("won.") || stdout.contains("draw!"),
            "{stdout}"
        );
        assert!(stdout.contains("Player 0 made move"), "{stdout}");
    }
}

#[test]
fn player_specific_options_are_rejected() {
    for args in [
        vec!["cli", "--depth", "2", "vs", "guessing"],
        vec!["minmax", "vs", "cli", "--depth", "2"],
        vec!["--size", "2", "--win-length", "3", "guessing", "vs", "cli"],
    ] {
        let output = game(&args);
        assert!(!output.status.success(), "{args:?}");
        assert!(String::from_utf8_lossy(&output.stderr).contains("error:"));
    }
}

#[test]
fn human_player_retries_invalid_moves() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_tic-tac-toe-mcts"))
        .args(["--size", "2", "--win-length", "2", "cli", "vs", "cli"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"bad\n0,0\n0,0\n0,1\n1,0\n")
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("row,column"));
    assert!(stderr.contains("already occupied"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Player 0 won."));
}

#[test]
fn human_player_reports_end_of_input() {
    let child = Command::new(env!("CARGO_BIN_EXE_tic-tac-toe-mcts"))
        .args(["cli", "vs", "guessing"])
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("Unexpected end of input")
    );
}
