use crate::git::Branch;
use crate::git::GitParseError;
use crate::git::Line;

#[derive(Debug, Clone, PartialEq)]
pub enum Head<'a> {
    Branch(Branch<'a>),
    Detached,
}

impl<'a> Head<'a> {
    pub(super) fn new(line: &Line<'a>) -> Result<Self, GitParseError> {
        match line.components() {
            ["(HEAD", ..] => Ok(Head::Detached),
            _ => Ok(Head::Branch(Branch::new(line)?)),
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
    let line = Line::parse("* (HEAD detached at 1f02cc2) 1f02cc2 Initial commit");

    let sut = Head::new(&line).unwrap();

    let expected = Head::Detached;

    assert_eq!(sut, expected);
}

#[test]
fn regular_branch() {
    let line = Line::parse("*  main  1f02cc2 [origin/main: ahead by 2] Initial commit");

    let sut = Head::new(&line).unwrap();

    let expected = head! { tracking {"main", remote("main", "origin", diverged)} };

    assert_eq!(sut, expected);
}
