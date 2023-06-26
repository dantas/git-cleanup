use crate::execute;
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
            print_clean_help();
            return;
        }
    };

    for action in actions::actions(&repository) {
        match action {
            actions::Action::Skip { branch_name } => {
                println!("Skipping branch {}", branch_name);
            }

            actions::Action::Delete { branch_name } => {
                if !delete_branch(path, branch_name, arg) {
                    break;
                }
            }
        }
    }
}

pub fn print_clean_help() {
    println!("clean options:");
    println!("    -step: Ask for user confirmation before deleting each branch");
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
        println!(
            "An error occurred while deleting branch {}, aborting cleanup",
            branch_name
        )
    }

    result.is_ok()
}

fn notify_step(branch_name: &str) -> Continue {
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

mod args {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum Arg {
        Step,
        NoMode,
    }

    pub fn parse(args: &[&str]) -> Option<Arg> {
        match args {
            ["--step"] => Some(Arg::Step),
            [] => Some(Arg::NoMode),
            _ => None,
        }
    }

    #[test]
    fn test_step_arg() {
        let sut = parse(&["--step"]);
        let expected = Some(Arg::Step);
        assert_eq!(sut, expected);
    }

    #[test]
    fn test_default_arg() {
        let sut = parse(&[]);
        let expected = Some(Arg::NoMode);
        assert_eq!(sut, expected);
    }

    #[test]
    fn test_invalid_arg() {
        let sut = parse(&["--invalid"]);
        let expected = None;
        assert_eq!(sut, expected);
    }
}

mod actions {
    use crate::git::Branch;
    use crate::git::Repository;
    use std::iter::Iterator;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Action<'a> {
        Skip { branch_name: &'a str },
        Delete { branch_name: &'a str },
    }

    pub fn actions<'a>(repository: &'a Repository) -> impl Iterator<Item = Action<'a>> {
        repository
            .branches
            .iter()
            .filter_map(|branch| match branch {
                Branch::Local { name } => {
                    let item = if *branch == repository.current_branch {
                        Action::Skip { branch_name: name }
                    } else {
                        Action::Delete { branch_name: name }
                    };

                    Some(item)
                }
                Branch::Tracked { .. } => None,
                Branch::Detached => None,
            })
    }

    #[test]
    fn test_actions() {
        use std::collections::HashSet;

        let repository = crate::git::repository! {
            *local_branch("develop"),
            tracked_branch { "main", remote_branch("main", "origin") },
            local_branch("feature"),
        };

        let sut = HashSet::from_iter(actions(&repository));

        let expected = HashSet::from([
            Action::Skip {
                branch_name: "develop",
            },
            Action::Delete {
                branch_name: "feature",
            },
        ]);

        assert_eq!(sut, expected);
    }
}
