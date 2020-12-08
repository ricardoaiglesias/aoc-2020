use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_14::day_14_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 14 Solution", |b|
        b.iter(|| day_14_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
