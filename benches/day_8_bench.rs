use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_8::{day_8_soln_bench, setup, silver, gold};

fn benchmark(bench: &mut Criterion) {
    bench.bench_function("Day 8 Solution", |b|
        b.iter(|| day_8_soln_bench())
    );

    bench.bench_function("Day 8 Setup", |b| b.iter(|| setup() ));

    // // Setup code for silver/gold.
    let mut info = setup();

    bench.bench_function("Day 8 Silver", |b| b.iter(|| silver(&mut info) ));
    bench.bench_function("Day 8 Gold", |b| b.iter(|| { gold(&mut info) } ));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
