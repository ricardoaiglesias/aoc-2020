use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_13::day_13_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 13 Solution", |b|
        b.iter(|| day_13_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
