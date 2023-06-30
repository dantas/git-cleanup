use crate::git::{Branch, Head, RemoteBranch, RemoteBranchStatus, Repository};
use std::fmt::Display;
use std::iter::Iterator;

pub fn list(repository: &Repository, args: &[&str]) {
    if args == ["--help"] {
        print_list_help();
        return;
    }

    let args = match args::parse(args) {
        Some(mode) => mode,
        None => {
            println!("option not recognized");
            print_list_help();
            return;
        }
    };

    match args {
        args::Arg::Local => print_local(repository),
        args::Arg::Tracked => print_tracked(repository),
        args::Arg::Gone => print_gone(repository),
        args::Arg::Diverged => print_diverged(repository),
        args::Arg::All => {
            print_local(repository);
            print_tracked(repository);
        }
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

mod args {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Arg {
        Local,
        Tracked,
        All,
        Gone,
        Diverged,
    }

    pub fn parse(args: &[&str]) -> Option<Arg> {
        match args {
            ["--all"] => Some(Arg::All),
            ["--tracked"] => Some(Arg::Tracked),
            ["--local"] => Some(Arg::Local),
            ["--gone"] => Some(Arg::Gone),
            ["--diverged"] => Some(Arg::Diverged),
            [] => Some(Arg::Gone), // default option if no arg is provided
            _ => None,
        }
    }

    #[test]
    fn local() {
        let sut = parse(&["--local"]);
        let expected = Some(Arg::Local);
        assert_eq!(sut, expected);
    }

    #[test]
    fn tracked() {
        let sut = parse(&["--tracked"]);
        let expected = Some(Arg::Tracked);
        assert_eq!(sut, expected);
    }

    #[test]
    fn all() {
        let sut = parse(&["--all"]);
        let expected = Some(Arg::All);
        assert_eq!(sut, expected);
    }

    #[test]
    fn gone() {
        let sut = parse(&["--gone"]);
        let expected = Some(Arg::Gone);
        assert_eq!(sut, expected);
    }

    #[test]
    fn diverged() {
        let sut = parse(&["--diverged"]);
        let expected = Some(Arg::Diverged);
        assert_eq!(sut, expected);
    }

    #[test]
    fn test_default_arg() {
        let sut = parse(&[]);
        let expected = Some(Arg::Gone);
        assert_eq!(sut, expected);
    }

    #[test]
    fn test_invalid_arg() {
        let sut = parse(&["invalid"]);
        let expected = None;
        assert_eq!(sut, expected);
    }
}
