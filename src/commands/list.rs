use std::fmt::Display;
use crate::git::{Branch, Repository};

pub fn list(repository: &Repository, args: &[&str]) -> bool {
    if args == ["--help"] {
        print_list_help();
        return true;
    }

    let mode = match parse_mode(args) {
        Some(mode) => mode,
        None => return false,
    };

    let iter = repository.branches
        .iter()
        .filter(|branch| { should_print(branch, mode) });

    for branch in iter {
        if branch == &repository.current_branch {
            print!("* ")
        }

        println!("{}", branch);
    }

    true
}

pub fn print_list_help() {
    println!(
        "List branches:
            --all:      List all branches
            --tracked:  List all tracked branches
            --local:    List local branches (default option)
        "
    )
}

fn parse_mode(args: &[&str]) -> Option<Mode> {
    match args {
        ["--all"] => Some(Mode::All),
        ["--tracked"] => Some(Mode::Tracked),
        ["--local"] => Some(Mode::Local),
        [] => Some(Mode::Local), // default option if no arg is provided
        _ => None
    }
}

fn should_print(branch: &Branch, mode: Mode) -> bool {
    mode == Mode::All || match branch {
        &Branch::Tracked { .. } => mode == Mode::Tracked,
        &Branch::Local { .. } => mode == Mode::Local,
        &Branch::Detached => false,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Local,
    Tracked,
    All
}

impl Display for Branch {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Branch::Tracked { name, remote } => {
                write!(formatter, "{} => {}/{}", name, remote.remote, remote.name)
            },
            Branch::Local { name } => write!(formatter, "{}", name),
            Branch::Detached => write!(formatter, "Detached"),
        }
    }
}