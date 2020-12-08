use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_21::day_21_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 21 Solution", |b|
        b.iter(|| day_21_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
