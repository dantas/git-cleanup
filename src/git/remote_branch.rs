#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct RemoteBranch {
    pub name: String,
    pub remote: String,
}

impl RemoteBranch {
    pub(super) fn try_from_vv_column(string: &str) -> Option<RemoteBranch> {
        let index_slash;

        if let Some(i) = string.find('/') {
            index_slash = i;
        } else {
            return None;
        }

        if !string.starts_with("[") || !string.ends_with("]") {
            return None;
        }

        let index_ending = if let Some(index_colon) = string.find(':') {
            index_colon
        } else {
            string.len() - 1
        };

        let remote_branch = RemoteBranch {
            remote: string[1..index_slash].to_owned(),
            name: string[index_slash + 1..index_ending].to_owned(),
        };

        Some(remote_branch)
    }
}

impl std::fmt::Display for RemoteBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Branch: {} - Origin: {}", self.name, self.remote)
    }
}

#[test]
fn test_remote_branch() {
    let expected = remote_branch!("branch2", "origin");

    if let Some(sut) = RemoteBranch::try_from_vv_column("[origin/branch2]") {
        assert_eq!(sut, expected)
    } else {
        panic!("try_from_vv_column didn't detect valid string");
    }
}

#[test]
fn test_remote_branch_ahead_of_origin() {
    let expected = remote_branch!("main", "origin");

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
macro_rules! remote_branch {
    ($branch_name:literal, $remote_name:literal) => {
        $crate::git::RemoteBranch {
            name: $branch_name.to_owned(),
            remote: $remote_name.to_owned(),
        }
    };
}

#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use remote_branch;
