use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use tic_tac_toe_mcts::interfaces::{GameState, PointPlacement, TicTacToeReferee};
use tic_tac_toe_mcts::referee::{FasterRefereeV1, NaiveReferee};

fn bench_fibs(c: &mut Criterion) {
    let board = [[None; 7]; 7];
    let mut board = GameState::new_with_values(board).unwrap();
    let placements = [
        PointPlacement { row: 3, column: 3 },
        PointPlacement { row: 0, column: 0 },
    ];
    let winning_length = 4;

    let naive_referee = NaiveReferee::new(winning_length);
    let faster_referee = FasterRefereeV1::new(winning_length);

    let mut group = c.benchmark_group("TicTacToe Referee (empty board)");
    for cur in &placements {
        group.bench_with_input(BenchmarkId::new("NaiveReferee", cur), cur, |b, &pp| {
            b.iter(|| {
                naive_referee.receive_move(&mut board, pp, 0);
                board[pp] = None.into();
            });
        });
        group.bench_with_input(
            BenchmarkId::new("FasterRefereeV1", cur),
            cur,
            |b, &pp| {
                b.iter(|| {
                    faster_referee.receive_move(&mut board, pp, 0);
                    board[pp] = None.into();
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
