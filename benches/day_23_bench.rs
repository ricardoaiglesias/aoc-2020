use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_23::day_23_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 23 Solution", |b|
        b.iter(|| day_23_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
