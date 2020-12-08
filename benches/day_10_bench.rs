use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_10::day_10_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 10 Solution", |b|
        b.iter(|| day_10_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
