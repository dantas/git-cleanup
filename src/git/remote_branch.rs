#[derive(Debug, Clone, PartialEq)]
pub struct RemoteBranch {
    pub name: String,
    pub remote: String,
}

impl RemoteBranch {
    pub fn try_from_vv_column(string: &str) -> Option<RemoteBranch> {
        let index;

        if let Some(i) = string.find('/') {
            index = i;
        } else {
            return None;
        }

        if !string.starts_with("[") || !string.ends_with("]") {
            return None;
        }

        let remote_branch = RemoteBranch {
            remote: string[1..index].to_owned(),
            name: string[index+1..string.len()-1].to_owned(), 
        };

        return Some(remote_branch)
    }    
}

impl std::fmt::Display for RemoteBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Branch: {} - Origin: {}", self.name, self.remote)?;
        std::fmt::Result::Ok(())
    }
}

#[test]
fn test_remote_branch_from_vv() {
    if let Some(remote_branch) = RemoteBranch::try_from_vv_column("[origin/branch2]") {
        assert_eq!(remote_branch.name, "branch2");
        assert_eq!(remote_branch.remote, "origin");
        return;
    }

    panic!("try_from_vv_column didn't detect valid string");
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
