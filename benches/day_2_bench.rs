use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_2::day_2_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 2 Solution", |b|
        b.iter(|| day_2_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
