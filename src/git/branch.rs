use super::line_parser::{LineComponents, LineParser};
use super::{GitParseError, RemoteBranch};

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
    pub(super) fn new(parser: &mut impl LineParser<'a>) -> Result<Self, GitParseError> {
        match parser.consume_components() {
            Some(components) => Ok(Branch::from_components(components)),
            None => Err(GitParseError::BranchPattern {
                line: parser.line().to_string(),
            }),
        }
    }

    fn from_components(component: LineComponents<'a>) -> Self {
        match RemoteBranch::parse(component.maybe_origin_branch) {
            Some(remote) => Branch::Tracking {
                name: component.branch_name,
                remote,
            },
            _ => Branch::Local {
                name: component.branch_name,
            },
        }
    }
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

#[test]
fn tracking_branch() {
    let mut parser = super::line_parser::new_line_parser(
        "main 1f02cc2 [origin/main: ahead by 2] Initial commit",
    );

    let sut = Branch::new(&mut parser).unwrap();

    let expected = tracking! {"main", remote("main", "origin", diverged)};

    assert_eq!(sut, expected);
}

#[test]
fn local_branch() {
    let mut parser = super::line_parser::new_line_parser("develop 1f02cc2 Initial commit");

    let sut = Branch::new(&mut parser).unwrap();

    let expected = local!("develop");

    assert_eq!(sut, expected);
}
