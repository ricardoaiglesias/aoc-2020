use criterion::{criterion_group, criterion_main, Criterion};
use aoc_2020::day_9::{day_9_soln, day_9_soln_hash, setup, silver, silver_hash };

fn benchmark(bench: &mut Criterion) {
    let mut data = setup();
    bench.bench_function("Day 9 Silver (Vec)", |b|
        b.iter(|| silver(&mut data))
    );

    bench.bench_function("Day 9 Silver (HashSet)", |b|
        b.iter(|| silver_hash(&mut data))
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

/*
Day 9 Silver (Vec)      time:   [106.81 us 106.96 us 107.12 us]
Day 9 Silver (HashSet)  time:   [572.06 us 572.99 us 574.11 us]
*/
