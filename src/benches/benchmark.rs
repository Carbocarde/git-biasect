// Benchmark the single-threaded runtime cost of an allocator.
// multicore runtime cost is found in tests/alloc_tests.
// TODO: Bench a real git repo
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use git_biasect::alloc::{BasicAllocator, DumbAllocator};
use git_biasect::tests::alloc_bencher::run_bench;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    let iters = 25;

    let mut one_thousand_commits_eight_runners =
        c.benchmark_group("1000 Commits 8 Runners Allocator");
    one_thousand_commits_eight_runners.bench_function("dumb", |b| {
        b.iter(|| {
            run_bench::<DumbAllocator>(
                black_box(1000),
                black_box(8),
                black_box(100.0),
                black_box(1.0),
                black_box(iters),
                black_box(true),
            )
        })
    });
    one_thousand_commits_eight_runners.bench_function("basic", |b| {
        b.iter(|| {
            run_bench::<BasicAllocator>(
                black_box(1000),
                black_box(8),
                black_box(100.0),
                black_box(1.0),
                black_box(iters),
                black_box(true),
            )
        })
    });
    one_thousand_commits_eight_runners.finish();
    let mut one_hundred_commits_eight_runners =
        c.benchmark_group("100 Commits 8 Runners Allocator");
    one_hundred_commits_eight_runners.bench_function("dumb", |b| {
        b.iter(|| {
            run_bench::<DumbAllocator>(
                black_box(100),
                black_box(8),
                black_box(100.0),
                black_box(1.0),
                black_box(iters),
                black_box(true),
            )
        })
    });
    one_hundred_commits_eight_runners.bench_function("basic", |b| {
        b.iter(|| {
            run_bench::<BasicAllocator>(
                black_box(100),
                black_box(8),
                black_box(100.0),
                black_box(1.0),
                black_box(iters),
                black_box(true),
            )
        })
    });

    one_hundred_commits_eight_runners.finish();
    let mut one_hundred_commits_one_runner = c.benchmark_group("100 Commits 1 Runner Allocator");
    one_hundred_commits_one_runner.bench_function("dumb", |b| {
        b.iter(|| {
            run_bench::<DumbAllocator>(
                black_box(100),
                black_box(1),
                black_box(100.0),
                black_box(1.0),
                black_box(iters),
                black_box(true),
            )
        })
    });
    one_hundred_commits_one_runner.bench_function("basic", |b| {
        b.iter(|| {
            run_bench::<BasicAllocator>(
                black_box(100),
                black_box(1),
                black_box(100.0),
                black_box(1.0),
                black_box(iters),
                black_box(true),
            )
        })
    });
    one_hundred_commits_one_runner.finish();
}

criterion_group! {
    name=benches;
    config=Criterion::default().measurement_time(Duration::from_secs(10));
    targets=criterion_benchmark
}
criterion_main!(benches);
