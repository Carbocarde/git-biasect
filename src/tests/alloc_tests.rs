// Benchmarks of threaded performance (Ignoring main-thread scheduler cost)
use float_eq::assert_float_eq;

use crate::{
    alloc::{BasicAllocator, DumbAllocator},
    tests::alloc_bencher::run_bench,
};

#[test]
fn two_commits_one_runner_dumb_bookends() {
    let res = run_bench::<DumbAllocator>(2, 1, 100.0, 1.0, 1000, true);

    let expected_steps = 2000;
    assert_eq!(
        res.1, expected_steps,
        "Dumb allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 202376.44, r2nd <= 0.000_1);
}

#[test]
fn two_commits_one_runner_basic_bookends() {
    let res = run_bench::<BasicAllocator>(2, 1, 100.0, 1.0, 1000, true);

    let expected_steps = 2000;
    assert_eq!(
        res.1, expected_steps,
        "Basic allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 202376.44, r2nd <= 0.000_1);
}

#[test]
fn one_thousand_commits_eight_runners_dumb_bookends() {
    let res = run_bench::<DumbAllocator>(1000, 8, 100.0, 1.0, 100, true);

    let expected_steps = 4756;
    assert_eq!(
        res.1, expected_steps,
        "Dumb allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 470921.06, r2nd <= 0.000_1);
}

#[test]
fn one_thousand_commits_eight_runners_basic_bookends() {
    let res = run_bench::<BasicAllocator>(1000, 8, 100.0, 1.0, 100, true);

    let expected_steps = 1273;
    assert_eq!(
        res.1, expected_steps,
        "Basic allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 126346.02, r2nd <= 0.000_1);
}

#[test]
fn one_thousand_commits_eight_runners_dumb() {
    let res = run_bench::<DumbAllocator>(1000, 8, 100.0, 1.0, 100, false);

    let expected_steps = 3640;
    assert_eq!(
        res.1, expected_steps,
        "Dumb allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 360359.05, r2nd <= 0.000_1);
}

#[test]
fn one_thousand_commits_eight_runners_basic() {
    let res = run_bench::<BasicAllocator>(1000, 8, 100.0, 1.0, 100, false);

    let expected_steps = 1148;
    assert_eq!(
        res.1, expected_steps,
        "Basic allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 113924.75, r2nd <= 0.000_1);
}

#[test]
fn one_hundred_commits_eight_runners_dumb_bookends() {
    let res = run_bench::<DumbAllocator>(100, 8, 100.0, 1.0, 100, true);

    let expected_steps = 1097;
    assert_eq!(
        res.1, expected_steps,
        "Dumb allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 109046.75, r2nd <= 0.000_1);
}

#[test]
fn one_hundred_commits_eight_runners_basic_bookends() {
    let res = run_bench::<BasicAllocator>(100, 8, 100.0, 1.0, 100, true);

    let expected_steps = 836;
    assert_eq!(
        res.1, expected_steps,
        "Basic allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 83156.53, r2nd <= 0.000_1);
}

#[test]
fn one_hundred_commits_eight_runners_dumb() {
    let res = run_bench::<DumbAllocator>(100, 8, 100.0, 1.0, 100, false);

    let expected_steps = 875;
    assert_eq!(
        res.1, expected_steps,
        "Dumb allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 87109.42, r2nd <= 0.000_1);
}

#[test]
fn one_hundred_commits_eight_runners_basic() {
    let res = run_bench::<BasicAllocator>(100, 8, 100.0, 1.0, 100, false);

    let expected_steps = 853;
    assert_eq!(
        res.1, expected_steps,
        "Basic allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 84911.47, r2nd <= 0.000_1);
}

#[test]
fn one_hundred_commits_one_runner_dumb_bookends() {
    let res = run_bench::<DumbAllocator>(100, 1, 100.0, 1.0, 100, true);

    let expected_steps = 5488;
    assert_eq!(
        res.1, expected_steps,
        "Dumb allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 548731.38, r2nd <= 0.000_1);
}

#[test]
fn one_hundred_commits_one_runner_basic_bookends() {
    let res = run_bench::<BasicAllocator>(100, 1, 100.0, 1.0, 100, true);

    let expected_steps = 870;
    assert_eq!(
        res.1, expected_steps,
        "Basic allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 87349.15, r2nd <= 0.000_1);
}

#[test]
fn one_hundred_commits_one_runner_dumb() {
    let res = run_bench::<DumbAllocator>(100, 1, 100.0, 1.0, 100, false);

    let expected_steps = 2838;
    assert_eq!(
        res.1, expected_steps,
        "Dumb allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 284002.17, r2nd <= 0.000_1);
}

#[test]
fn one_hundred_commits_one_runner_basic() {
    let res = run_bench::<BasicAllocator>(100, 1, 100.0, 1.0, 100, false);

    let expected_steps = 670;
    assert_eq!(
        res.1, expected_steps,
        "Basic allocator does not match expected # of steps ({expected_steps})"
    );
    assert_float_eq!(res.0, 67240.03, r2nd <= 0.000_1);
}
