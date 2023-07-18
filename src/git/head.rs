use super::line_parser::LineParser;
use super::{Branch, GitParseError};

#[derive(Debug, Clone, PartialEq)]
pub enum Head<'a> {
    Branch(Branch<'a>),
    Detached,
}

impl<'a> Head<'a> {
    pub(super) fn new(parser: &mut impl LineParser<'a>) -> Result<Self, GitParseError> {
        if parser.consume_if_detached() {
            Ok(Head::Detached)
        } else {
            Ok(Head::Branch(Branch::new(parser)?))
        }
    }
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! head {
    ( detached $($ignore:tt)* ) => {
        crate::git::Head::Detached
    };

    ( $branch_type:ident $args:tt ) => {
        crate::git::Head::Branch($crate::git::branch! { $branch_type $args })
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use head;

#[test]
fn detached_head() {
    let mut parser =
        super::line_parser::new_line_parser("(HEAD detached at 1f02cc2) 1f02cc2 Initial commit");

    let sut = Head::new(&mut parser).unwrap();

    let expected = Head::Detached;

    assert_eq!(sut, expected);
}

#[test]
fn regular_branch() {
    let mut parser = super::line_parser::new_line_parser(
        "main  1f02cc2 [origin/main: ahead by 2] Initial commit",
    );

    let sut = Head::new(&mut parser).unwrap();

    let expected = head! { tracking {"main", remote("main", "origin", diverged)} };

    assert_eq!(sut, expected);
}
