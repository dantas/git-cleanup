use std::collections::HashSet;

use crate::git::Branch;
use crate::git::GitError;

#[derive(Debug, Clone, PartialEq)]
pub struct Repository<'a> {
    pub current_branch: Branch<'a>,
    pub branches: HashSet<Branch<'a>>,
}

impl<'a> Repository<'a> {
    pub(super) fn from_vv_output(command_stdout: &'a str) -> Result<Self, GitError> {
        let mut branches = HashSet::new();
        let mut current_branch = None;

        for line in command_stdout.lines() {
            let result = Branch::from_vv_line(line)?;

            if result.is_current {
                current_branch = Some(result.branch.clone())
            }

            branches.insert(result.branch);
        }

        match current_branch {
            Some(current_branch) => Ok(Repository {
                current_branch,
                branches,
            }),
            None => Err(GitError::CurrentBranch),
        }
    }
}

#[test]
fn test_one_branch() {
    let sut = Repository::from_vv_output("* main 73b4084 [origin/main] commit message").unwrap();

    let expected = repository! {
        *tracked_branch { "main", remote_branch("main", "origin") }
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_multiple_branches() {
    let sut = Repository::from_vv_output(
        "\
        * main 73b4084 [origin/main] commit message\n\
        develop 73b4084 [origin/develop] commit message\
    ",
    )
    .unwrap();

    let expected = repository! {
        *tracked_branch { "main" , remote_branch("main", "origin") },
        tracked_branch { "develop" , remote_branch("develop", "origin") },
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_local_branch() {
    let sut = Repository::from_vv_output(
        "\
        * main 73b4084 [origin/main] commit message\n\
        local 73b4084 commit message\
    ",
    )
    .unwrap();

    let expected = repository! {
        *tracked_branch { "main", remote_branch("main", "origin") },
        local_branch("local"),
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_dettached_branch() {
    let sut = Repository::from_vv_output(
        "\
        * (HEAD detached at 1f02cc2) 1f02cc2 Initial commit\n\
        local 73b4084 commit message\
    ",
    )
    .unwrap();

    let expected = repository! {
        *detached,
        local_branch("local"),
    };

    assert_eq!(sut, expected);
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! repository {
    ( * $type:ident $args:tt $( , $rest_type:ident $rest_args:tt )* $(,)? ) => {
        {
            let current_branch = $crate::git::make_branch!($type $args);

            let mut branches = std::collections::HashSet::new();

            branches.insert(current_branch.clone());

            $(
                branches.insert(
                    $crate::git::make_branch!{ $rest_type $rest_args }
                );
            )*

            crate::git::Repository {
                current_branch,
                branches,
            }
        }
    };

    ( * $type:ident $( , $rest_type:ident $rest_args:tt )* $(,)? ) => {
        {
            let current_branch = $crate::git::make_branch!($type);

            let mut branches = std::collections::HashSet::new();

            branches.insert(current_branch.clone());

            $(
                branches.insert(
                    $crate::git::make_branch!{ $rest_type $rest_args }
                );
            )*

            crate::git::Repository {
                current_branch,
                branches,
            }
        }
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use repository;
