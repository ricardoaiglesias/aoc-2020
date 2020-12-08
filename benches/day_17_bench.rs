use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_17::day_17_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 17 Solution", |b|
        b.iter(|| day_17_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
