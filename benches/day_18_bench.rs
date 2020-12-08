use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_18::day_18_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 18 Solution", |b|
        b.iter(|| day_18_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
