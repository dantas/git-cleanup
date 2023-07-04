use super::line::Line;
use super::{GitParseError, RemoteBranch};

#[cfg(test)]
use crate::git;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Branch<'a> {
    Tracking {
        name: &'a str,
        remote: RemoteBranch<'a>,
    },

    Local {
        name: &'a str,
    },
}

impl<'a> Branch<'a> {
    pub(super) fn new(line: &Line<'a>) -> Result<Self, GitParseError> {
        match line.components() {
            [branch_name, _, maybe_origin_branch, ..] => {
                Ok(Branch::from_components(branch_name, maybe_origin_branch))
            }
            _ => Err(GitParseError::BranchPattern {
                line: line.to_string(),
            }),
        }
    }

    fn from_components(branch_name: &'a str, maybe_origin_branch: &'a str) -> Self {
        let remote_branch = RemoteBranch::parse(maybe_origin_branch);

        match remote_branch {
            Some(remote) => Branch::Tracking {
                name: branch_name,
                remote,
            },
            _ => Branch::Local { name: branch_name },
        }
    }
}

#[test]
fn tracking_branch() {
    let line = Line::parse("*  main  1f02cc2 [origin/main: ahead by 2] Initial commit");

    let sut = Branch::new(&line).unwrap();

    let expected = git::tracking! {"main", remote("main", "origin", diverged)};

    assert_eq!(sut, expected);
}

#[test]
fn local_branch() {
    let line = Line::parse("develop    1f02cc2 Initial commit");

    let sut = Branch::new(&line).unwrap();

    let expected = git::local!("develop");

    assert_eq!(sut, expected);
}

#[test]
fn test_parse_invalid_lines() {
    assert!(Branch::new(&Line::parse(" ")).is_err());

    assert!(Branch::new(&Line::parse("first")).is_err());

    assert!(Branch::new(&Line::parse("* first")).is_err());

    assert!(Branch::new(&Line::parse("* first second")).is_err());
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! tracking {
    ($name:literal, remote ( $remote_name:literal, $remote_origin: literal, $remote_status:ident ) ) => {
        $crate::git::Branch::Tracking {
            name: $name,
            remote: crate::git::remote!($remote_name, $remote_origin, $remote_status),
        }
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use tracking;

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! local {
    ($name:literal) => {
        $crate::git::Branch::Local { name: $name }
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use local;

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! branch {
    ( local $args:tt ) => {
        $crate::git::local!$args
    };

    ( tracking $args:tt ) => {
        $crate::git::tracking!$args
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use branch;
