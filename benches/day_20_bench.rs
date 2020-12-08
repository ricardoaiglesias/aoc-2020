use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_20::day_20_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 20 Solution", |b|
        b.iter(|| day_20_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
