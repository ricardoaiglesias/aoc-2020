use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_9::day_9_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 9 Solution", |b|
        b.iter(|| day_9_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
