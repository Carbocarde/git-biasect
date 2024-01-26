/// Functions that invoke shell commands
use std::{
    path::Path,
    process::{Child, Command},
};

use crate::alloc::Status;

/// Get commits. Ordered from old to new.
pub fn get_commits(repo_path: &Path) -> Result<Vec<String>, String> {
    let cmd_git_bisect_log = Command::new("git")
        .arg("-C")
        .arg(repo_path.as_os_str().to_str().unwrap())
        .arg("bisect")
        .arg("visualize")
        .arg("--oneline")
        .arg("--reverse")
        .output()
        .unwrap();
    let out = String::from_utf8(cmd_git_bisect_log.stdout).unwrap();
    let lines = out.lines();
    let hashes: Vec<Option<&str>> = lines.map(|s| s.split(' ').next()).collect();

    if hashes.contains(&None) {
        return Err(format!("Could not get commit range from output: '{out}'"));
    }

    let unwrapped_hashes: Vec<String> =
        hashes.into_iter().map(|x| x.unwrap().to_string()).collect();

    if unwrapped_hashes.is_empty() {
        return Err("No hashes in bisection range. Did you set the bounds of your bisection with `git bisect good` and `git bisect bad`?".to_string());
    }

    // This unwrapped_hashes has all hashes that could be or are known to be bad.

    // Get the hash that is one above the current range (known to be good).
    let good_hash = {
        // Get known good commit
        let cmd_git_log = Command::new("git")
            .arg("-C")
            .arg(repo_path.as_os_str().to_str().unwrap())
            .arg("log")
            .arg("--oneline")
            .arg("-n")
            .arg("1")
            .arg(format!("{}^", unwrapped_hashes.first().unwrap()))
            .output()
            .unwrap();

        let out = String::from_utf8(cmd_git_log.stdout).unwrap();
        let lines = out.lines();
        let good_hash: Option<&str> = lines.map(|s| s.split(' ').next()).next().unwrap();

        good_hash.unwrap().to_string()
    };

    let hashes = [vec![good_hash], unwrapped_hashes].concat();

    Ok(hashes)
}

pub fn reproducer_shell_commands(repo_path: &Path, command: &String, commit: &String) -> String {
    format!(
        "export TESTDIR=$(mktemp -d -t biasect.XXXXXX)\n\
        echo $TESTDIR\n\
        cd $TESTDIR\n\
        git -C {} worktree add $TESTDIR {commit} --detach\n\
        {command}\n\
        echo $?",
        repo_path.as_os_str().to_str().unwrap()
    )
}

pub fn run_script(repo_path: &Path, command: &str, commit: &String) -> Child {
    let tempdir_cmd = Command::new("mktemp")
        .arg("-d")
        .arg("-t")
        .arg("biasect.XXXXXX")
        .output();

    if let Err(err) = tempdir_cmd {
        panic!("Failed to create tempdir via `mktemp -d -t biasect`: {err}");
    }

    let tempdir_cmd = tempdir_cmd.unwrap();

    let tempdir = String::from_utf8_lossy(&tempdir_cmd.stdout)
        .trim()
        .to_string();

    // Chained commands (&&) in plaintext. I wish there was an easier way to do this.
    // 1. Checkout worktree
    // git worktree -C {repo_path} add {tempdir} {commit} --detach
    // 2. Invoke command
    Command::new("sh")
        .arg("-c")
        .arg(format!(
            "git -C {} worktree add {tempdir} {commit} --detach && {command}",
            repo_path.as_os_str().to_str().unwrap()
        ))
        .current_dir(tempdir)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Failed to execute script on hash") // Simulating script execution
}

/// git bisect <good|bad|skip> <hash>
pub fn bisect_report(repo_path: &Path, status: &Status, hash: &String) -> Child {
    let action = match status {
        Status::Good => "good",
        Status::Bad => "bad",
        Status::Skip => "skip",
        Status::Unknown => {
            panic!("Cannot report unknown state to git bisect. Valid choices are Good, Bad, Skip.")
        }
    };

    Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("bisect")
        .arg(action)
        .arg(hash)
        .current_dir(repo_path.as_os_str().to_str().unwrap())
        .spawn()
        .unwrap_or_else(|_| {
            panic!(
                "Failed to execute `git bisect {action} {hash}` at directory `{}`",
                repo_path.as_os_str().to_str().unwrap()
            )
        })
}

pub fn worktree_prune(repo_path: &Path) -> Child {
    Command::new("git")
        .arg("worktree")
        .arg("prune")
        .current_dir(repo_path.as_os_str().to_str().unwrap())
        .spawn()
        .unwrap_or_else(|_| {
            panic!(
                "Failed to execute `git worktree prune` at directory `{}`",
                repo_path.as_os_str().to_str().unwrap()
            )
        })
}
