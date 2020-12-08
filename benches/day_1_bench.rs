use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_1::day_1_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 1 Solution", |b|
        b.iter(|| day_1_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
