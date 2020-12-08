use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_16::day_16_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 16 Solution", |b|
        b.iter(|| day_16_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
