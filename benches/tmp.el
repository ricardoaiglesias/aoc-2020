(dotimes (n 25)
  (let ((filename (format "day_%d_bench.rs" n)))
    (find-file filename)
    (insert (format "use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2020::day_%d::day_%d_soln;

fn benchmark(bench: &mut Criterion) {
    bench.bench_function(\"Day %d Solution\", |b|
        b.iter(|| day_%d_soln())
    );
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
" n n n n))
    (save-buffer)))

(provide 'tmp)
;;; tmp.el ends here
