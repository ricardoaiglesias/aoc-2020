use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_3::day_3_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 3 Solution", |b|
        b.iter(|| day_3_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
