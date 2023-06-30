use crate::execute;
use crate::git::Branch;
use crate::git::RemoteBranch;
use crate::git::RemoteBranchStatus;
use crate::git::Repository;
use std::io;

pub fn clean<P: AsRef<std::path::Path>>(path: &P, repository: Repository, args: &[&str]) {
    if args == ["--help"] {
        print_clean_help();
        return;
    }

    let arg = match args::parse(args) {
        Some(mode) => mode,
        None => {
            println!("option not recognized");
            print_clean_help();
            return;
        }
    };

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
                if !delete_branch(path, name, arg) {
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

fn delete_branch<P: AsRef<std::path::Path>>(
    path: &P,
    branch_name: &str,
    arg: args::Arg,
) -> Continue {
    if arg == args::Arg::Step && !notify_step(branch_name) {
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

mod args {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Arg {
        Step,
        Automatic,
    }

    pub fn parse(args: &[&str]) -> Option<Arg> {
        match args {
            ["--step"] => Some(Arg::Step),
            ["--automatic"] => Some(Arg::Automatic),
            [] => Some(Arg::Step),
            _ => None,
        }
    }

    #[test]
    fn step_arg() {
        let sut = parse(&["--step"]);
        let expected = Some(Arg::Step);
        assert_eq!(sut, expected);
    }

    #[test]
    fn automatic_arg() {
        let sut = parse(&["--automatic"]);
        let expected = Some(Arg::Automatic);
        assert_eq!(sut, expected);
    }

    #[test]
    fn default_arg() {
        let sut = parse(&[]);
        let expected = Some(Arg::Step);
        assert_eq!(sut, expected);
    }

    #[test]
    fn invalid_arg() {
        let sut = parse(&["--invalid"]);
        let expected = None;
        assert_eq!(sut, expected);
    }
}
