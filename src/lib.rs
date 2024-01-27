pub mod alloc;
pub mod shell;
pub mod tests;
pub mod visualize;

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
    pub total: usize,
}

#[derive(Debug)]
pub struct State {
    pub runtime_samples: Vec<f64>,
    pub commits: Vec<CommitState>,
    pub runners: Runners,
    pub check_bookends: bool,
}
