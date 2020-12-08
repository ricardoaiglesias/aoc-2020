use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_24::day_24_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 24 Solution", |b|
        b.iter(|| day_24_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
