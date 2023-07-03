use crate::args::ListOption;
use crate::git::{Branch, Head, RemoteBranch, RemoteBranchStatus, Repository};
use std::fmt::Display;
use std::iter::Iterator;

pub fn list(repository: &Repository, option: &ListOption) {
    match option {
        ListOption::Help => print_list_help(),
        ListOption::All => {
            print_local(repository);
            print_tracked(repository);
        }
        ListOption::Local => print_local(repository),
        ListOption::Tracked => print_tracked(repository),
        ListOption::Gone => print_gone(repository),
        ListOption::Diverged => print_diverged(repository),
    }
}

pub fn print_list_help() {
    println!("list options:");
    println!("    --gone:     List tracking branches that are gone from origin (default option)");
    println!("    --diverged: List tracking branches that diverged from origin");
    println!("    --all:      List all local and tracked branches");
    println!("    --tracked:  List all tracked branches");
    println!("    --local:    List local branches");
}

fn print_local(repository: &Repository) {
    print_branches(repository, "Local branches", |b| {
        matches!(b, Branch::Local { .. })
    })
}

fn print_tracked(repository: &Repository) {
    print_branches(repository, "Tracked branches", |b| {
        matches!(b, Branch::Tracking { .. })
    })
}

fn print_gone(repository: &Repository) {
    print_branches(repository, "Gone branches", |b| {
        matches!(
            b,
            Branch::Tracking {
                remote: RemoteBranch {
                    status: RemoteBranchStatus::Gone,
                    ..
                },
                ..
            }
        )
    })
}

fn print_diverged(repository: &Repository) {
    print_branches(repository, "Diverged branches", |b| {
        matches!(
            b,
            Branch::Tracking {
                remote: RemoteBranch {
                    status: RemoteBranchStatus::Diverged,
                    ..
                },
                ..
            }
        )
    })
}

fn print_branches(repository: &Repository, message: &str, filter: impl Fn(&&Branch) -> bool) {
    println!("{message}:");

    if let Head::Branch(branch) = &repository.head {
        if filter(&branch) {
            println!("    *{}", &repository.head);
        }
    };

    for branch in repository.branches.iter().filter(filter) {
        println!("    {branch}");
    }
}

impl<'a> Display for Head<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Head::Branch(branch) => branch.fmt(formatter),
            Head::Detached => write!(formatter, "Detached"),
        }
    }
}

impl<'a> Display for Branch<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Branch::Tracking { name, remote } => {
                write!(
                    formatter,
                    "{} => {}/{}",
                    name, remote.remote_name, remote.branch_name
                )
            }
            Branch::Local { name } => write!(formatter, "{}", name),
        }
    }
}
