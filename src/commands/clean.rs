use crate::args::CleanOption;
use crate::execute;
use crate::git::{Branch, RemoteBranch, RemoteBranchStatus, Repository};
use std::io;
use std::path::Path;

pub fn clean(path: &Path, repository: Repository, option: &CleanOption) {
    if *option == CleanOption::Help {
        print_clean_help();
        return;
    }

    for branch in repository.branches.iter() {
        match branch {
            Branch::Tracking {
                name,
                remote:
                    RemoteBranch {
                        status: RemoteBranchStatus::Gone,
                        ..
                    },
                ..
            } => {
                if !delete_branch(path, name, option) {
                    break;
                }
            }
            _ => continue,
        }
    }
}

pub fn print_clean_help() {
    println!("clean options:");
    println!("    --step: Ask for user confirmation before deleting each branch (default option)");
    println!("    --automatic: Delete branches without asking for user output");
}

type Continue = bool;

fn delete_branch(path: &Path, branch_name: &str, option: &CleanOption) -> Continue {
    if *option == CleanOption::Step && !notify_step(branch_name) {
        return false;
    }

    let result = execute::execute(&path, &"git", &["branch", "-d", branch_name]);

    if result.is_err() {
        println!("An error occurred while deleting branch {branch_name}, aborting cleanup")
    }

    result.is_ok()
}

fn notify_step(branch_name: &str) -> Continue {
    println!("About to delete branch {branch_name}, type y and press enter to continue");

    let mut line = String::new();
    let result = io::stdin().read_line(&mut line);

    if result.is_err() {
        println!("An error occured, aborting cleanup");
        return false;
    }

    if line != "y\n" {
        println!("Understood, aborting cleanup");
        return false;
    }

    true
}
