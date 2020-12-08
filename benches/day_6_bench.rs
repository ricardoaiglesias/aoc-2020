use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_6::day_6_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 6 Solution", |b|
        b.iter(|| day_6_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
