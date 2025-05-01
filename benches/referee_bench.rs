use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use tic_tac_toe_mcts::interfaces::{GameState, PointPlacement, TicTacToeReferee};
use tic_tac_toe_mcts::referee::NaiveReferee;

fn bench_fibs(c: &mut Criterion) {
    let board = [[None; 7]; 7];
    let board = GameState::new_with_values(board).unwrap();
    let placements = [
        PointPlacement { row: 3, column: 3 },
        PointPlacement { row: 0, column: 0 },
    ];
    let winning_length = 4;

    let referee_v1 = NaiveReferee::new(winning_length);
    let referee_v2 = NaiveReferee::new(winning_length);

    let mut group = c.benchmark_group("TicTacToe Referee (empty board)");
    for cur in &placements {
        group.bench_with_input(BenchmarkId::new("Referee v1", cur), cur, |b, pp| {
            b.iter(|| {
                let mut board = board.clone();
                referee_v1.receive_move(&mut board, *pp, 0);
            });
        });
        group.bench_with_input(BenchmarkId::new("Referee v2", cur), cur, |b, pp| {
            b.iter(|| {
                let mut board = board.clone();
                referee_v2.receive_move(&mut board, *pp, 0);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
