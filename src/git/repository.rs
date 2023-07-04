use super::line::Line;
use super::{Branch, GitParseError, Head};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Repository<'a> {
    pub head: Head<'a>,
    pub branches: HashSet<Branch<'a>>,
}

impl<'a> Repository<'a> {
    pub(super) fn parse(command_stdout: &'a str) -> Result<Self, GitParseError> {
        let mut branches = HashSet::new();
        let mut head = None;

        for line in command_stdout.lines() {
            let line = Line::parse(line);

            if line.is_head() {
                head = Some(Head::new(&line)?);
            } else {
                branches.insert(Branch::new(&line)?);
            }
        }

        match head {
            Some(head) => Ok(Repository { head, branches }),
            None => Err(GitParseError::CurrentBranch),
        }
    }
}

#[test]
fn one_branch() {
    let sut = Repository::parse("* main 73b4084 [origin/main] commit message").unwrap();

    let expected = repository! {
        *tracking { "main", remote("main", "origin", synchronized) }
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_multiple_branches() {
    let sut = Repository::parse(
        "\
        * main 73b4084 [origin/main] commit message\n\
        develop 73b4084 [origin/develop] commit message\
    ",
    )
    .unwrap();

    let expected = repository! {
        *tracking { "main" , remote("main", "origin", synchronized) },
        tracking { "develop" , remote("develop", "origin", synchronized) },
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_local_branch() {
    let sut = Repository::parse(
        "\
        * main 73b4084 [origin/main] commit message\n\
        local 73b4084 commit message\
    ",
    )
    .unwrap();

    let expected = repository! {
        *tracking { "main", remote("main", "origin", synchronized) },
        local("local"),
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_dettached_branch() {
    let sut = Repository::parse(
        "\
        * (HEAD detached at 1f02cc2) 1f02cc2 Initial commit\n\
        local 73b4084 commit message\
    ",
    )
    .unwrap();

    let expected = repository! {
        *detached,
        local("local"),
    };

    assert_eq!(sut, expected);
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! repository {
    ( * $type:ident $args:tt $( , $rest_type:ident $rest_args:tt )* $(,)? ) => {
        {
            let head = $crate::git::head!($type $args);

            let branches = std::collections::HashSet::from([
                $($crate::git::branch!{ $rest_type $rest_args }),*
            ]);

            crate::git::Repository {
                head,
                branches,
            }
        }
    };

    ( * $type:ident $( , $rest_type:ident $rest_args:tt )* $(,)? ) => {
        {
            let head = $crate::git::head!($type $args);

            let branches = std::collections::HashSet::from([
                $($crate::git::branch!{ $rest_type $rest_args }),*
            ]);

            crate::git::Repository {
                head,
                branches,
            }
        }
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use repository;
