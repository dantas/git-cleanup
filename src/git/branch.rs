use crate::git::RemoteBranch;

use super::GitError;
use regex::Regex;

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
pub struct ParseBranchResult {
    pub branch: Branch,
    pub is_current: bool
}

impl Branch {
    pub fn from_vv_line(line: &str) -> Result<ParseBranchResult, GitError> {
        let components = split_components(line)?;

        let branch = match components.as_slice() {
            ["*", "(HEAD", ..] => { 
                ParseBranchResult {
                    branch: Self::Detached,
                    is_current: true,
                }
            }
            ["*", &ref branch_name, _, &ref maybe_origin_branch, ..] => {
                Branch::new_parse_branch_result(branch_name, maybe_origin_branch, true)
            }
            [&ref branch_name, _, &ref maybe_origin_branch, ..] if branch_name != "*" => {
                Branch::new_parse_branch_result(branch_name, maybe_origin_branch, false)
            }
            _ => return Result::Err(GitError::new_with_string(format!("String format not recognized {}", line)))
        };

        Result::Ok(branch)
    }

    fn new_parse_branch_result(branch_name: &str, maybe_origin_branch: &str, is_current: bool) -> ParseBranchResult {
        let remote_branch = RemoteBranch::try_from_vv_column(maybe_origin_branch);

        ParseBranchResult {
            branch: Branch::new(branch_name.to_owned(), remote_branch),
            is_current,
        }
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

fn split_components(line: &str) -> Result<Vec<&str>, GitError> {
    let regex = Regex::new(r"(\[.*\])+|(\S)+")?;

    let captures_iter = regex
        .captures_iter(line)
        .filter_map(|c| c.get(0))
        .map(|m| m.as_str());

    let vec= Vec::from_iter(captures_iter);

    Result::Ok(vec)
}

impl From<regex::Error> for GitError {
    fn from(source: regex::Error) -> Self {
        GitError::new_with_source(Box::new(source))
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
            branch: Branch::Tracked {
                name: "main".to_owned(),
                remote: RemoteBranch {
                    name: "main".to_owned(),
                    remote: "origin".to_owned(),
                }
            },
            is_current: true
        };

    assert_eq!(
        sut,
        expected
    );
}

#[test]
fn test_parse_tracked_branch() {
    let sut = Branch::from_vv_line("develop    1f02cc2 Initial commit").unwrap();

    let expected =
        ParseBranchResult {
            branch: Branch::Local {
                name: "develop".to_owned(),
            },
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