use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_11::day_11_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 11 Solution", |b|
        b.iter(|| day_11_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
