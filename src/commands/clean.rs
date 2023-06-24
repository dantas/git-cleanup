use crate::execute;
use crate::git::Branch;
use crate::git::Repository;
use std::io;

pub fn clean<P: AsRef<std::path::Path>>(path: P, repository: Repository, args: &[&str]) {
    if args == ["--help"] {
        print_clean_help();
        return;
    }

    let mode = match parse_mode(args) {
        Some(mode) => mode,
        None => {
            print_clean_help();
            return;
        }
    };

    for branch in &repository.branches {
        match branch {
            Branch::Local { name: branch_name } => {
                if skip_branch(branch, branch_name, &repository) {
                    continue;
                }

                if !delete_branch(&path, branch_name, mode) {
                    break;
                }
            }
            _ => continue,
        };
    }
}

pub fn print_clean_help() {
    println!("clean options:");
    println!("    -step: Ask for user confirmation before deleting each branch");
}

fn skip_branch(branch: &Branch, branch_name: &str, repository: &Repository) -> bool {
    let skip = branch == &repository.current_branch;

    if skip {
        println!("Skipping branch {}", branch_name);
    }

    skip
}

fn delete_branch<P: AsRef<std::path::Path>>(path: P, branch_name: &str, mode: Mode) -> bool {
    if mode == Mode::Step && !notify_step(branch_name) {
        return false;
    }

    let result = execute::execute(&path, "git", ["branch", "-d", branch_name]);

    if result.is_err() {
        println!(
            "An error occurred while deleting branch {}, aborting cleanup",
            branch_name
        )
    }

    result.is_ok()
}

fn notify_step(branch_name: &str) -> bool {
    println!(
        "About to delete branch {}, type y and press enter to continue",
        branch_name
    );

    let mut line = String::new();
    let result = io::stdin().read_line(&mut line);

    if result.is_err() {
        println!("An error occured, aborting cleanup");
        return false;
    }

    if line != "y" {
        println!("Understood, aborting cleanup");
        return false;
    }

    true
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Step,
    NoMode,
}

fn parse_mode(args: &[&str]) -> Option<Mode> {
    match args {
        ["--step"] => Some(Mode::Step),
        [] => Some(Mode::NoMode),
        _ => None,
    }
}
