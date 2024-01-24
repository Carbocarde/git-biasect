use crate::alloc::Status;

pub fn print_commits(commits: &[Status], runners: &[usize]) {
    // TODO: Limit the range of commits to print.
    // Helpful if there are a large # of commits removed from consideration.
    // Maybe a syntax like: [100] G..R...B [800]
    // [100] G..[80]..R..[32]..R..[40]..B [900]
    for (idx, x) in commits.iter().enumerate() {
        if runners.contains(&idx) {
            print!("R");
        } else {
            match x {
                Status::Good => print!("G"),
                Status::Bad => print!("B"),
                Status::Skip => print!("S"),
                Status::Unknown => print!("."),
            }
        }
    }

    println!();
}
