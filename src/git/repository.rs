use super::{Branch, GitParseError, GitQuery, Head, LineParser};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub struct Repository<'a> {
    pub head: Head<'a>,
    pub branches: HashSet<Branch<'a>>,
}

impl<'a> Repository<'a> {
    pub(super) fn parse(query: &'a GitQuery) -> Result<Self, GitParseError> {
        // Here we assume that count_lines() is cheaper than creating hashset with the wrong capacity and having to allocate memory twice
        let mut branches = HashSet::with_capacity(query.count_lines());
        let mut head = None;

        for mut parser in query.lines() {
            if parser.consume_if_head() {
                head = Some(Head::new(&mut parser)?);
            } else {
                branches.insert(Branch::new(&mut parser)?);
            }
        }

        match head {
            Some(head) => Ok(Repository { head, branches }),
            None => Err(GitParseError::CurrentBranch),
        }
    }
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

#[test]
fn one_branch() {
    let query = GitQuery("* main 73b4084 [origin/main] commit message".to_string());

    let sut = Repository::parse(&query).unwrap();

    let expected = repository! {
        *tracking { "main", remote("main", "origin", synchronized) }
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_multiple_branches() {
    let query = GitQuery(
        "* main 73b4084 [origin/main] commit message\n\
         develop 73b4084 [origin/develop] commit message"
            .to_string(),
    );

    let sut = Repository::parse(&query).unwrap();

    let expected = repository! {
        *tracking { "main" , remote("main", "origin", synchronized) },
        tracking { "develop" , remote("develop", "origin", synchronized) },
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_local_branch() {
    let query = GitQuery(
        "* main 73b4084 [origin/main] commit message\n\
         local 73b4084 commit message"
            .to_string(),
    );

    let sut = Repository::parse(&query).unwrap();

    let expected = repository! {
        *tracking { "main", remote("main", "origin", synchronized) },
        local("local"),
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_dettached_branch() {
    let query = GitQuery(
        "* (HEAD detached at 1f02cc2) 1f02cc2 Initial commit\n\
         local 73b4084 commit message"
            .to_string(),
    );

    let sut = Repository::parse(&query).unwrap();

    let expected = repository! {
        *detached,
        local("local"),
    };

    assert_eq!(sut, expected);
}
