use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_25::day_25_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 25 Solution", |b|
        b.iter(|| day_25_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
