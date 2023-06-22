use crate::git::GitError;
use crate::git::RemoteBranch;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Branch {
    Tracked { name: String, remote: RemoteBranch },

    Local { name: String },

    Detached,
}

#[derive(Debug, PartialEq)]
pub(super) struct ParseBranchResult {
    pub branch: Branch,
    pub is_current: bool,
}

impl Branch {
    pub(super) fn from_vv_line(line: &str) -> Result<ParseBranchResult, GitError> {
        let components = split_components(line)?;

        let branch = match components.as_slice() {
            ["*", "(HEAD", ..] => {
                ParseBranchResult {
                    branch: Branch::Detached,
                    is_current: true,
                }
            }
            ["*", &ref branch_name, _, &ref maybe_origin_branch, ..] => {
                ParseBranchResult {
                    branch: Self::from_components(branch_name, maybe_origin_branch),
                    is_current: true,
                }
            }
            [&ref branch_name, _, &ref maybe_origin_branch, ..] if branch_name != "*" => {
                ParseBranchResult {
                    branch: Self::from_components(branch_name, maybe_origin_branch),
                    is_current: false,
                }
            }
            _ => {
                return Err(GitError::BranchPattern {
                    line: line.to_string(),
                });
            }
        };

        Ok(branch)
    }

    fn from_components(branch_name: &str, maybe_origin_branch: &str) -> Branch {
        let remote_branch = RemoteBranch::try_from_vv_column(maybe_origin_branch);

        let name = branch_name.to_string();

        match remote_branch {
            Some(remote) => Branch::Tracked { name, remote },
            _ => Branch::Local { name },
        }
    }
}

fn split_components(line: &str) -> Result<Vec<&str>, GitError> {
    let regex = Regex::new(r"(\[.*\])+|(\S)+")?;

    let captures_iter = regex
        .captures_iter(line)
        .filter_map(|c| c.get(0))
        .map(|m| m.as_str());

    let vec = Vec::from_iter(captures_iter);

    Ok(vec)
}

#[test]
fn test_parse_detached_head() {
    let sut = Branch::from_vv_line("* (HEAD detached at 1f02cc2) 1f02cc2 Initial commit").unwrap();

    let expected = ParseBranchResult {
        branch: Branch::Detached,
        is_current: true,
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_parse_currently_checked_out_tracked_branch() {
    let sut =
        Branch::from_vv_line("*  main  1f02cc2 [origin/main: ahead by 2] Initial commit").unwrap();

    let expected = ParseBranchResult {
        branch: tracked_branch! {"main", remote_branch("main", "origin")},
        is_current: true,
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_parse_local_branch() {
    let sut = Branch::from_vv_line("develop    1f02cc2 Initial commit").unwrap();

    let expected = ParseBranchResult {
        branch: local_branch!("develop"),
        is_current: false,
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_parse_invalid_lines() {
    assert!(Branch::from_vv_line(" ").is_err());

    assert!(Branch::from_vv_line("first").is_err());

    assert!(Branch::from_vv_line("* first").is_err());

    assert!(Branch::from_vv_line("* first second").is_err());
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! tracked_branch {
    ($name:literal, remote_branch ( $remote_name:literal, $remote_origin: literal ) ) => {
        $crate::git::Branch::Tracked {
            name: $name.to_owned(),
            remote: crate::git::remote_branch!($remote_name, $remote_origin),
        }
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use tracked_branch;

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! local_branch {
    ($name:literal) => {
        $crate::git::Branch::Local {
            name: $name.to_owned(),
        }
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use local_branch;

#[cfg(test)]
#[allow(unused_macros)]
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
#[allow(unused_imports)]
pub(crate) use make_branch;
