use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_12::{day_12_soln, setup, silver, gold,  silver_gold_iter};

fn benchmark(bench: &mut Criterion) {
    
    bench.bench_function("Day 12 Bigboy", |b|
        b.iter(|| day_12_soln())
    );

    bench.bench_function("Day 12 Bigboy (Lazy Iterator)", |b|
                         b.iter(|| silver_gold_iter()));

    // bench.bench_function("Day 12 Setup", |b|
    //                      b.iter(|| setup())
    // );

    // let data = setup();
    // bench.bench_function("Day 12 Silver", |b|
    //                      b.iter(|| silver(&data))
    // );

    // bench.bench_function("Day 12 Gold", |b|
    //                      b.iter(|| gold(&data))
    // );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
