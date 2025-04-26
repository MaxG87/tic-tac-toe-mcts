use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn fibonacci(n: u64) -> u64 {
    let mut fib = 1;
    for n in 1..=n {
        fib += n;
    }
    fib
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
