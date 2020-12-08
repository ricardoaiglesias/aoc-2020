use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_22::day_22_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 22 Solution", |b|
        b.iter(|| day_22_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
