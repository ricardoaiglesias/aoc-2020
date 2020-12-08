use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_5::day_5_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 5 Solution", |b|
        b.iter(|| day_5_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
