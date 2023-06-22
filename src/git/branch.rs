use regex::Regex;
use crate::git::RemoteBranch;
use crate::error::Error;
use crate::error;

#[derive(Debug, Clone, PartialEq)]
pub enum Branch {
    Tracked {
        name: String,
        remote: RemoteBranch,
    },

    Local {
        name: String,
    },

    Detached
}

#[derive(Debug, PartialEq)]
pub(super) struct ParseBranchResult {
    pub branch: Branch,
    pub is_current: bool
}

impl ParseBranchResult {
    fn new(branch_name: &str, maybe_origin_branch: &str, is_current: bool) -> ParseBranchResult {
        let remote_branch = RemoteBranch::try_from_vv_column(maybe_origin_branch);

        ParseBranchResult {
            branch: Branch::new(branch_name.to_owned(), remote_branch),
            is_current,
        }
    }

    fn new_detached() -> Self {
        ParseBranchResult {
            branch: Branch::Detached,
            is_current: true,
        }
    }
}

impl Branch {
    pub(super) fn from_vv_line(line: &str) -> Result<ParseBranchResult, Error> {
        let components = split_components(line)?;

        let branch = match components.as_slice() {
            ["*", "(HEAD", ..] => { 
                ParseBranchResult::new_detached()
            }
            ["*", &ref branch_name, _, &ref maybe_origin_branch, ..] => {
                ParseBranchResult::new(branch_name, maybe_origin_branch, true)
            }
            [&ref branch_name, _, &ref maybe_origin_branch, ..] if branch_name != "*" => {
                ParseBranchResult::new(branch_name, maybe_origin_branch, false)
            }
            _ => return Result::Err(error::new_error_with_string!("String format not recognized {}", line))
        };

        Ok(branch)
    }

    fn new(name: String, remote_branch: Option<RemoteBranch>) -> Self {
        match remote_branch {
            Some(remote_branch) => {
                Branch::Tracked { name, remote: remote_branch }
            }
            _ => {
                Branch::Local { name }
            }
        }
    }
}

fn split_components(line: &str) -> Result<Vec<&str>, Error> {
    let regex = Regex::new(r"(\[.*\])+|(\S)+")?;

    let captures_iter = regex
        .captures_iter(line)
        .filter_map(|c| c.get(0))
        .map(|m| m.as_str());

    let vec= Vec::from_iter(captures_iter);

    Ok(vec)
}

impl From<regex::Error> for Error {
    fn from(source: regex::Error) -> Self {
        Error::new_with_source(Box::new(source))
    }
}

#[test]
fn test_parse_detached_head() {
    let sut = Branch::from_vv_line("* (HEAD detached at 1f02cc2) 1f02cc2 Initial commit").unwrap();

    let expected =
        ParseBranchResult {
            branch: Branch::Detached,
            is_current: true,
        };

    assert_eq!(
        sut,
        expected
    );
}

#[test]
fn test_parse_currently_checked_out_tracked_branch() {
    let sut = Branch::from_vv_line("*  main  1f02cc2 [origin/main: ahead by 2] Initial commit").unwrap();

    let expected =
        ParseBranchResult {
            branch: tracked_branch!{"main", remote_branch("main", "origin")},
            is_current: true
        };

    assert_eq!(
        sut,
        expected
    );
}

#[test]
fn test_parse_local_branch() {
    let sut = Branch::from_vv_line("develop    1f02cc2 Initial commit").unwrap();

    let expected =
        ParseBranchResult {
            branch: local_branch!("develop"),
            is_current: false,
        };

    assert_eq!(
        sut,
        expected
    );


}

#[test]
fn test_parse_invalid_lines() {
    assert!(Branch::from_vv_line(" ").is_err());

    assert!(Branch::from_vv_line("first").is_err());
    
    assert!(Branch::from_vv_line("* first").is_err());
    
    assert!(Branch::from_vv_line("* first second").is_err());
}

#[cfg(test)]
macro_rules! tracked_branch {
    ($name:literal, remote_branch ( $remote_name:literal, $remote_origin: literal ) ) => {
        $crate::git::Branch::Tracked {
            name: $name.to_owned(),
            remote: crate::git::remote_branch!($remote_name, $remote_origin),
        }
    };
}

#[cfg(test)]
pub(crate) use tracked_branch;

#[cfg(test)]
macro_rules! local_branch {
    ($name:literal) => {
        $crate::git::Branch::Local {
            name: $name.to_owned(),
        }
    };
}

#[cfg(test)]
pub(crate) use local_branch;

#[cfg(test)]
macro_rules! make_branch {
    ( detached ) => {
        $crate::git::Branch::Detached
    };

    ( local_branch $args:tt ) => {
        $crate::git::local_branch!$args
    };

    ( tracked_branch $args:tt ) => {
        $crate::git::tracked_branch!$args
    };
}

#[cfg(test)]
pub(crate) use make_branch;