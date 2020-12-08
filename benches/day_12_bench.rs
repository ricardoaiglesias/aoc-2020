use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_12::day_12_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 12 Solution", |b|
        b.iter(|| day_12_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
