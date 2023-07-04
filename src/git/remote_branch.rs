use super::RemoteBranchStatus;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct RemoteBranch<'a> {
    pub branch_name: &'a str,
    pub remote_name: &'a str,
    pub status: RemoteBranchStatus,
}

impl<'a> RemoteBranch<'a> {
    pub(super) fn parse(string: &'a str) -> Option<Self> {
        if !RemoteBranch::validate(string) {
            return None;
        }

        let brackless_string = &string[1..string.len() - 1];

        let (left, right) = match brackless_string.split_once(':') {
            Some(pair) => pair,
            None => (brackless_string, ""),
        };

        let (remote_name, branch_name) = RemoteBranch::parse_names(left);
        let status = RemoteBranchStatus::parse(right);

        Some(RemoteBranch {
            remote_name,
            branch_name,
            status,
        })
    }

    fn validate(string: &str) -> bool {
        string.starts_with('[') && string.ends_with(']') && string.find('/').is_some()
    }

    fn parse_names(left: &str) -> (&str, &str) {
        // SAFETY: validate() already checked that '/' exists here
        let index_slash = left.find('/').unwrap();
        (&left[..index_slash], &left[index_slash + 1..])
    }
}

impl<'a> std::fmt::Display for RemoteBranch<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Branch: {} - Origin: {}",
            self.branch_name, self.remote_name
        )
    }
}

#[test]
fn test_remote_branch() {
    let expected = remote!("branch2", "origin", synchronized);

    let sut = RemoteBranch::parse("[origin/branch2]").unwrap();

    assert_eq!(sut, expected);
}

#[test]
fn test_remote_branch_ahead_of_origin() {
    let expected = remote!("main", "origin", diverged);

    let sut = RemoteBranch::parse("[origin/main: ahead 1]").unwrap();

    assert_eq!(sut, expected);
}

#[test]
fn test_parse_invalid_lines() {
    if RemoteBranch::parse("origin/branch2]").is_some() {
        panic!("try_from_vv_column interpreted missing [ as a valid remote branch");
    }

    if RemoteBranch::parse("[origin/branch2").is_some() {
        panic!("try_from_vv_column interpreted missing ] as a valid remote branch");
    }

    if RemoteBranch::parse("originbranch2]").is_some() {
        panic!("try_from_vv_column interpreted missing / as a valid remote branch");
    }
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! remote {
    ($branch_name:literal, $remote_name:literal, $status:ident) => {
        $crate::git::RemoteBranch {
            branch_name: $branch_name,
            remote_name: $remote_name,
            status: $crate::git::remote_status!($status),
        }
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use remote;
