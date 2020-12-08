use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_4::day_4_soln;

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 4 Solution", |b|
        b.iter(|| day_4_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
