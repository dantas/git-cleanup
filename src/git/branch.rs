use crate::git::RemoteBranch;

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
    pub fn parse_from_vv_line(line: &str) -> Option<ParseBranchResult> {
        let words: Vec<&str> = line.split_whitespace().collect();

        let branch = match words.as_slice() {
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
            _ => return None
        };

        Some(branch)
    }

    fn new_parse_branch_result(branch_name: &str, maybe_origin_branch: &str, is_current: bool) -> ParseBranchResult {
        let remote_branch = RemoteBranch::parse_from_vv_column(maybe_origin_branch);

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

#[test]
fn test_parse_detached_head() {
    let sut = Branch::parse_from_vv_line("* (HEAD detached at 1f02cc2) 1f02cc2 Initial commit");

    let expected =
        ParseBranchResult {
            branch: Branch::Detached,
            is_current: true,
        };

    assert_eq!(
        sut,
        Some(expected)
    );
}

#[test]
fn test_parse_currently_checked_out_tracked_branch() {
    let sut = Branch::parse_from_vv_line("*  main  1f02cc2 [origin/main] Initial commit");

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
        Some(expected)
    );
}

#[test]
fn test_parse_tracked_branch() {
    let sut = Branch::parse_from_vv_line("develop    1f02cc2 Initial commit");

    let expected =
        ParseBranchResult {
            branch: Branch::Local {
                name: "develop".to_owned(),
            },
            is_current: false,
        };

    assert_eq!(
        sut,
        Some(expected)
    );


}

#[test]
fn test_parse_invalid_lines() {
    assert_eq!(
        Branch::parse_from_vv_line(" "),
        None
    );

    assert_eq!(
        Branch::parse_from_vv_line("first"),
        None
    );

    assert_eq!(
        Branch::parse_from_vv_line("* first"),
        None
    );

    assert_eq!(
        Branch::parse_from_vv_line("* first second"),
        None
    );
}