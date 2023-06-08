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
    // TODO: Check if all cases are covered. Export error telling what happened?
    pub fn parse_from_vv_line(line: &str) -> Option<ParseBranchResult> {
        let words: Vec<&str> = line.split_whitespace().collect();

        let branch = match words.as_slice() {
            ["*", "(HEAD", ..] => {
                ParseBranchResult {
                    branch: Branch::Detached,
                    is_current: true,
                }
            }
            // TODO: Merge two branches below into one
            ["*", &ref branch_name, _, maybe_origin_branch, ..] => {
                let remote_branch = RemoteBranch::parse_from_vv_column(maybe_origin_branch);

                ParseBranchResult {
                    branch: Branch::new(branch_name.to_owned(), remote_branch),
                    is_current: true
                }
            },
            [&ref branch_name, _, maybe_origin_branch, ..] => {
                let remote_branch = RemoteBranch::parse_from_vv_column(maybe_origin_branch);

                ParseBranchResult {
                    branch: Branch::new(branch_name.to_owned(), remote_branch),
                    is_current: false,
                }
            },
            _ => return None
        };

        Some(branch)
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

// TODO: Split one test into multiple ones
#[test]
fn test_branch_parse_from_vv_line() {
    let detached = Branch::parse_from_vv_line("* (HEAD detached at 1f02cc2) 1f02cc2 Initial commit");

    assert_eq!(
        detached,
        Some(
            ParseBranchResult {
                branch: Branch::Detached,
                is_current: true,
            }
        )
    );

    // == 

    let tracked = Branch::parse_from_vv_line("*  main  1f02cc2 [origin/main] Initial commit");

    assert_eq!(
        tracked,
        Some(
            ParseBranchResult {
                branch: Branch::Tracked {
                    name: "main".to_owned(),
                    remote: RemoteBranch {
                        name: "main".to_owned(),
                        remote: "origin".to_owned(),
                    }
                },
                is_current: true
            }
        )
    );

    // == 

    let local = Branch::parse_from_vv_line("develop    1f02cc2 Initial commit");

    assert_eq!(
        local,
        Some(
            ParseBranchResult {
                branch: Branch::Local {
                    name: "develop".to_owned(),
                },
                is_current: false,
            }
        )
    );

    let empty = Branch::parse_from_vv_line("  ");

    assert_eq!(empty, None);
}
