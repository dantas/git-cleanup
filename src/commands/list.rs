use crate::git::{Branch, Repository};
use std::fmt::Display;
use std::iter::Iterator;

pub fn list(repository: &Repository, args: &[&str]) {
    if args == ["--help"] {
        print_list_help();
        return;
    }

    let mode = match parse_mode(args) {
        Some(mode) => mode,
        None => {
            print_list_help();
            return;
        }
    };

    match mode {
        Mode::Local => print_local(repository),
        Mode::Tracked => print_tracked(repository),
        Mode::All => {
            print_local(repository);
            print_tracked(repository);
        }
    }
}

pub fn print_list_help() {
    println!("list options:");
    println!("    --all:      List all branches");
    println!("    --tracked:  List all tracked branches");
    println!("    --local:    List local branches (default option)");
}

fn print_local(repository: &Repository) {
    print_branches(repository, "Local branches", |b| {
        matches!(b, Branch::Local { .. })
    })
}

fn print_tracked(repository: &Repository) {
    print_branches(repository, "Tracked branches", |b| {
        matches!(b, Branch::Tracked { .. })
    })
}

fn print_branches<F: Fn(&&Branch) -> bool>(repository: &Repository, message: &str, filter: F) {
    println!("{}:", message);

    for branch in repository.branches.iter().filter(filter) {
        if branch == &repository.current_branch {
            println!("    *{}", branch);
        } else {
            println!("    {}", branch);
        }
    }
}

fn parse_mode(args: &[&str]) -> Option<Mode> {
    match args {
        ["--all"] => Some(Mode::All),
        ["--tracked"] => Some(Mode::Tracked),
        ["--local"] => Some(Mode::Local),
        [] => Some(Mode::Local), // default option if no arg is provided
        _ => None,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Local,
    Tracked,
    All,
}

impl Display for Branch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Branch::Tracked { name, remote } => {
                write!(formatter, "{} => {}/{}", name, remote.remote, remote.name)
            }
            Branch::Local { name } => write!(formatter, "{}", name),
            Branch::Detached => write!(formatter, "Detached"),
        }
    }
}
