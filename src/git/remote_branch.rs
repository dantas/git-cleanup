#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct RemoteBranch<'a> {
    pub branch_name: &'a str,
    pub remote_name: &'a str,
}

impl<'a> RemoteBranch<'a> {
    pub(super) fn try_from_vv_column(string: &'a str) -> Option<Self> {
        let index_slash;

        if let Some(i) = string.find('/') {
            index_slash = i;
        } else {
            return None;
        }

        if !string.starts_with('[') || !string.ends_with(']') {
            return None;
        }

        let index_ending = if let Some(index_colon) = string.find(':') {
            index_colon
        } else {
            string.len() - 1
        };

        let remote_branch = RemoteBranch {
            remote_name: &string[1..index_slash],
            branch_name: &string[index_slash + 1..index_ending],
        };

        Some(remote_branch)
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
    let expected = remote!("branch2", "origin");

    if let Some(sut) = RemoteBranch::try_from_vv_column("[origin/branch2]") {
        assert_eq!(sut, expected)
    } else {
        panic!("try_from_vv_column didn't detect valid string");
    }
}

#[test]
fn test_remote_branch_ahead_of_origin() {
    let expected = remote!("main", "origin");

    if let Some(sut) = RemoteBranch::try_from_vv_column("[origin/main: ahead 1]") {
        assert_eq!(sut, expected)
    } else {
        panic!("try_from_vv_column didn't detect valid string");
    }
}

#[test]
fn test_parse_invalid_lines() {
    if RemoteBranch::try_from_vv_column("origin/branch2]") != None {
        panic!("try_from_vv_column interpreted missing [ as a valid remote branch");
    }

    if RemoteBranch::try_from_vv_column("[origin/branch2") != None {
        panic!("try_from_vv_column interpreted missing ] as a valid remote branch");
    }

    if RemoteBranch::try_from_vv_column("originbranch2]") != None {
        panic!("try_from_vv_column interpreted missing / as a valid remote branch");
    }
}

#[cfg(test)]
#[allow(unused_macros)]
macro_rules! remote {
    ($branch_name:literal, $remote_name:literal) => {
        $crate::git::RemoteBranch {
            branch_name: $branch_name,
            remote_name: $remote_name,
        }
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use remote;
