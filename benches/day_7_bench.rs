use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_7::day_7_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 7 Solution", |b|
        b.iter(|| day_7_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
