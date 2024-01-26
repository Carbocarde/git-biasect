use std::collections::HashSet;
use tested_trait::{test_impl, tested_trait};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Status {
    Good,
    Bad,
    Skip,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct CommitState {
    pub hash: String,
    pub status: Status,
}

#[derive(Debug)]
pub struct Runners {
    /// Runner to commit mapping
    pub commits: Vec<usize>,
    /// Runner start time
    pub start_times: Vec<f64>,
    total: usize,
}

#[derive(Debug)]
pub struct State {
    runtime_samples: Vec<f64>,
    pub commits: Vec<CommitState>,
    pub runners: Runners,
    check_bookends: bool,
}

// I'm pretty sure this is optimal for all common cases
// There might be a better allocation by doubling-up on certain commits if somehow variance is very high and mean is low. Seems unlikely.
fn initial_alloc(commits: &Vec<String>, runners: usize, check_bookends: bool) -> Vec<usize> {
    let mut runners_to_allocate = runners;

    if commits.len() <= runners_to_allocate {
        return (0..commits.len()).collect();
    } else if runners_to_allocate == 0 {
        return vec![];
    }

    let mut lower_bound = 0;
    let mut upper_bound = commits.len() - 1;

    let mut new_runners = HashSet::new();

    if check_bookends {
        if runners_to_allocate == 1 {
            new_runners.insert(upper_bound);
            upper_bound -= 1;
            runners_to_allocate -= 1;
        } else if runners_to_allocate >= 2 {
            new_runners.insert(lower_bound);
            new_runners.insert(upper_bound);
            upper_bound -= 1;
            lower_bound += 1;
            runners_to_allocate -= 2;
        }
    }

    if runners_to_allocate != 0 {
        let spacing = (upper_bound - lower_bound) / (runners_to_allocate + 1);

        new_runners.extend((0..runners_to_allocate).map(|x| x * spacing + spacing + lower_bound))
    }

    new_runners.into_iter().collect()
}

pub fn init(commits: &Vec<String>, runners: usize, check_bookends: bool) -> State {
    let runner_commits = initial_alloc(commits, runners, check_bookends);
    let runner_start_times = runner_commits.iter().map(|_| 0.0).collect();

    State {
        runtime_samples: vec![],
        commits: commits
            .iter()
            .map(|x| CommitState {
                hash: x.clone(),
                status: Status::Unknown,
            })
            .collect(),
        runners: Runners {
            commits: runner_commits,
            start_times: runner_start_times,
            total: runners,
        },
        check_bookends,
    }
}

fn invalidate_runners(
    runners: &[usize],
    index: usize,
    status: Status,
) -> (Vec<usize>, HashSet<usize>) {
    match status {
        Status::Good => (
            runners.iter().filter(|x| x > &&index).copied().collect(),
            runners.iter().filter(|x| x <= &&index).copied().collect(),
        ),
        Status::Bad => (
            runners.iter().filter(|x| x < &&index).copied().collect(),
            runners.iter().filter(|x| x >= &&index).copied().collect(),
        ),
        Status::Skip => {
            let invalidated_runners = if runners.contains(&index) {
                vec![index]
            } else {
                vec![]
            };
            (
                runners.iter().filter(|x| x != &&index).copied().collect(),
                invalidated_runners.iter().copied().collect(),
            )
        }
        Status::Unknown => (runners.to_vec(), HashSet::new()),
    }
}

/// Returns starting commit idx and slice
pub fn get_range(commits: &Vec<CommitState>) -> (usize, &[CommitState]) {
    let oldest_good_idx = commits
        .iter()
        .enumerate()
        .rev()
        .find(|(_, x)| x.status == Status::Good)
        .map(|(i, _)| i);
    let newest_bad_idx = &commits
        .iter()
        .enumerate()
        .find(|(_, x)| x.status == Status::Bad)
        .map(|(i, _)| i);

    let interesting_start = oldest_good_idx.map(|x| x + 1);
    let interesting_end = newest_bad_idx;

    (
        interesting_start.unwrap_or(0),
        &commits[interesting_start.unwrap_or(0)..interesting_end.unwrap_or(commits.len())],
    )
}

pub fn step<F>(
    state: &State,
    status: Status,
    index: usize,
    runtime: f64,
    time: f64,
) -> (State, HashSet<usize>, Vec<usize>)
where
    F: Allocator,
{
    assert!(
        runtime.is_sign_positive(),
        "Runtime is non-positive: {runtime}"
    );
    assert!(time.is_sign_positive(), "Time is non-positive: {time}");

    let commits = state
        .commits
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if i == index {
                CommitState {
                    hash: c.hash.clone(),
                    status,
                }
            } else {
                c.clone()
            }
        })
        .collect();

    let (remaining_runners, invalidated_runners) =
        invalidate_runners(&state.runners.commits, index, status);
    let bisection_range = get_range(&commits);
    let new_runners = F::alloc_runners(
        state.runners.total,
        &remaining_runners,
        bisection_range,
        state.check_bookends,
    );

    assert!(
        new_runners.iter().all(|x| bisection_range.0 <= *x),
        "Allocator scheduled known-good commit. Runners: {:?}, lower bound: {}",
        new_runners,
        bisection_range.0
    );
    assert!(
        new_runners
            .iter()
            .all(|x| *x < bisection_range.0 + bisection_range.1.len()),
        "Allocator scheduled known-bad commit. Runners: {:?}, upper bound: {}",
        new_runners,
        bisection_range.0 + bisection_range.1.len() - 1
    );

    let mut runners = vec![];
    runners.extend(remaining_runners);
    runners.extend(&new_runners);

    assert!(
        runners.len() <= state.runners.total,
        "{:?} !<= {} max runners: {}",
        runners,
        runners.len(),
        state.runners.total
    );
    let runner_start_times = runners.iter().map(|_| time).collect();

    let runners = Runners {
        commits: runners,
        start_times: runner_start_times,
        total: state.runners.total,
    };

    assert!(
        !runners.commits.is_empty() || get_range(&commits).1.is_empty(),
        "Scheduler fail! Commits remaining with no runners scheduled. Runners commit indexes: {:?} Commit range: {:?}",
        runners.commits,
        get_range(&state.commits).1
    );

    (
        State {
            runtime_samples: [state.runtime_samples.clone(), vec![runtime]].concat(),
            commits,
            runners,
            check_bookends: state.check_bookends,
        },
        invalidated_runners,
        new_runners,
    )
}

#[tested_trait]
pub trait Allocator {
    fn alloc_runners(
        runners: usize,
        existing_alloc: &[usize],
        bisection_range: (usize, &[CommitState]),
        check_bookends: bool,
    ) -> Vec<usize>;

    #[test]
    fn alloc_respects_range_offset() {
        let runners = 1;
        let existing_alloc = vec![];
        let commit_range = vec![CommitState {
            hash: "ONLY_COMMIT".to_string(),
            status: Status::Unknown,
        }];
        let bisection_range = (12, commit_range.as_slice());
        let allocated_runners =
            Self::alloc_runners(runners, &existing_alloc, bisection_range, false);

        assert!(allocated_runners.len() == 1);
        let expected_runner = 12;
        assert!(
            allocated_runners.contains(&expected_runner),
            "Allocator failed to respect bisection range offset. {:?} != {{{expected_runner}}}",
            allocated_runners
        );
    }

    #[test]
    fn alloc_respects_bounds_checking() {
        let runners = 1;
        let existing_alloc = vec![];
        let commit_range = vec![
            CommitState {
                hash: "GOOD_COMMIT".to_string(),
                status: Status::Unknown,
            },
            CommitState {
                hash: "BAD_COMMIT".to_string(),
                status: Status::Unknown,
            },
            CommitState {
                hash: "BAD_COMMIT".to_string(),
                status: Status::Unknown,
            },
            CommitState {
                hash: "BAD_COMMIT".to_string(),
                status: Status::Unknown,
            },
        ];
        let bisection_range = (0, commit_range.as_slice());
        let allocated_runners =
            Self::alloc_runners(runners, &existing_alloc, bisection_range, true);

        assert!(allocated_runners.len() == 1);
        assert!(allocated_runners.contains(&0));
    }
}

pub struct DumbAllocator;
#[test_impl]
impl Allocator for DumbAllocator {
    fn alloc_runners(
        runners: usize,
        existing_alloc: &[usize],
        bisection_range: (usize, &[CommitState]),
        check_bookends: bool,
    ) -> Vec<usize> {
        let mut bounds_start = bisection_range.0;
        let bounds_end = bisection_range.0 + bisection_range.1.len();

        if (bisection_range.1.len() as i64) <= runners as i64 {
            // We can allocate everything!
            return (bounds_start..bounds_end)
                .filter(|x| !existing_alloc.contains(x))
                .collect();
        }

        let mut new_runners = HashSet::new();
        let mut new_runners_to_allocate = runners - existing_alloc.len();

        // If runners are >= 2, then the bounds would already be scheduled.
        if check_bookends && runners == 1 && bisection_range.0 == 0 {
            new_runners.insert(0);
            bounds_start += 1;
            new_runners_to_allocate -= 1;
        }

        // We have to make decisions :(
        // Dumbly just assign to the next elem
        new_runners.extend(
            (bounds_start..bounds_end)
                .filter(|x| !existing_alloc.contains(x))
                .take(new_runners_to_allocate),
        );

        new_runners.into_iter().collect()
    }
}

pub struct BasicAllocator;
#[test_impl]
impl Allocator for BasicAllocator {
    fn alloc_runners(
        runners: usize,
        existing_alloc: &[usize],
        bisection_range: (usize, &[CommitState]),
        check_bookends: bool,
    ) -> Vec<usize> {
        let mut bounds_start = bisection_range.0;
        let mut bounds_end = bisection_range.0 + bisection_range.1.len();

        if (bisection_range.1.len() as i64) <= runners as i64 {
            // We can allocate everything!
            return (bounds_start..bounds_end)
                .filter(|x| !existing_alloc.contains(x))
                .collect();
        }

        let mut new_runners = HashSet::new();
        let mut new_runners_to_allocate = runners - existing_alloc.len();

        // If runners are >= 2, then the bounds would already be scheduled.
        if check_bookends && runners == 1 && bisection_range.0 == 0 {
            new_runners.insert(0);
            bounds_start += 1;
            bounds_end -= 1;
            new_runners_to_allocate -= 1;
        }

        // We have to make decisions :(
        // Space new runners out equally over the range.
        let valid_additions = (bounds_start..bounds_end)
            .filter(|x| !existing_alloc.contains(x))
            .collect::<Vec<_>>();

        let spacing = valid_additions.len() / (new_runners_to_allocate + 1);

        let idxes: Vec<_> = (0..new_runners_to_allocate)
            .map(|x| x * spacing + spacing)
            .collect();

        new_runners.extend(
            idxes
                .into_iter()
                .filter(|x| !existing_alloc.contains(&(bounds_start + x)))
                .map(|x| bounds_start + x),
        );

        // Update remaining runners to allocate
        new_runners_to_allocate = runners - existing_alloc.len() - new_runners.len();

        // If after deduplicating we have runners, just greedily allocate them linearly
        if new_runners_to_allocate != 0 {
            // Dumbly allocate
            new_runners.extend(
                valid_additions
                    .into_iter()
                    .filter(|x| !(existing_alloc.contains(x) || new_runners.contains(x)))
                    .take(new_runners_to_allocate)
                    .collect::<Vec<usize>>(),
            );
        }

        new_runners.into_iter().collect()
    }
}
