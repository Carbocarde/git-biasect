use rand::{rngs::StdRng, Rng, SeedableRng};
use rand_distr::{Distribution, Normal};

use crate::alloc::{get_range, init, step, Allocator, State, Status};

fn generate_runtime_for_commits(
    commits: usize,
    runtime_mean: f64,
    runtime_stddev: f64,
) -> Vec<f64> {
    (0..commits)
        .map(|x| {
            let mut rng = StdRng::seed_from_u64(x.try_into().unwrap());
            let normal = Normal::new(runtime_mean, runtime_stddev).unwrap();
            normal.sample(&mut rng)
        })
        .map(|t| if t <= 0.0 { 0.001 } else { t })
        .collect()
}

fn get_next_result(
    runners: &[usize],
    runner_start_times: &[f64],
    commit_runtimes: &[f64],
) -> usize {
    // Get the min start time + run time. Return the commit index.
    *runners
        .iter()
        .zip(runner_start_times)
        .map(|(commit, start)| (commit, commit_runtimes.get(*commit).unwrap() - start))
        .min_by(|x, y| x.1.total_cmp(&y.1))
        .unwrap()
        .0
}

/// Returns runtime and steps
// TODO: Factor in allocator/stepper runtime
pub fn run_bench<F>(
    commit_count: i32,
    runners: usize,
    runtime_mean: f64,
    runtime_stddev: f64,
    iters: u64,
    check_bookends: bool,
) -> (f64, usize)
where
    F: Allocator,
{
    let mut total_runtime = 0.0;
    let mut total_steps = 0;
    for seed in 0..iters {
        let commits: Vec<_> = (0..commit_count).map(|n| n.to_string()).collect();

        let commit_runtimes =
            generate_runtime_for_commits(commits.len(), runtime_mean, runtime_stddev);

        let mut state: State = init(&commits, runners, check_bookends);

        let mut rng = StdRng::seed_from_u64(seed);
        let first_bad = rng.gen_range(0..commit_count - 1);
        let commit_truths: Vec<_> = (0..commit_count)
            .map(|n| {
                if n <= first_bad {
                    Status::Good
                } else {
                    Status::Bad
                }
            })
            .collect();

        let mut runtime;

        let mut steps = 0;

        loop {
            steps += 1;
            let res = get_next_result(
                &state.runners.commits,
                &state.runners.start_times,
                &commit_runtimes,
            );

            let start_time = state
                .runners
                .commits
                .iter()
                .zip(&state.runners.start_times)
                .find(|(commit, _start_time)| commit == &&res)
                .unwrap()
                .1;

            runtime = start_time + commit_runtimes.get(res).unwrap();

            let mut _invalidated_runners;
            let mut _new_runners;
            (state, _invalidated_runners, _new_runners) = step::<F>(
                &state,
                *commit_truths.get(res).unwrap(),
                res,
                *commit_runtimes.get(res).unwrap(),
                start_time + commit_runtimes.get(res).unwrap(),
            );

            if get_range(&state.commits).1.is_empty() {
                break;
            }
        }
        total_runtime += runtime;
        total_steps += steps;
    }
    (total_runtime, total_steps)
}
