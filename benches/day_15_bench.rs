use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_15::day_15_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 15 Solution", |b|
        b.iter(|| day_15_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
