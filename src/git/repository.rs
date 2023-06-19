use crate::git::Branch;
use crate::git::GitError;
use crate::execute;

#[allow(unused_imports)]
use crate::git::RemoteBranch;

#[derive(Debug, Clone, PartialEq)]
pub struct Repository {
    pub current_branch: Branch,
    pub branches: Vec<Branch>
}

pub fn query_repository<P : AsRef<std::path::Path>> (dir: P) -> Result<Repository, GitError> {
    let branch_vv_stdout: String = execute::execute(dir, "git",["branch", "-vv"])?;
    Repository::from_vv_stdout(branch_vv_stdout)
}

impl Repository {
    fn from_vv_stdout<S : AsRef<str>>(command_stdout: S) -> Result<Repository, GitError> {
        let mut branches = Vec::new();
        let mut current_branch = None;
    
        for line in command_stdout.as_ref().lines() {
            let result = Branch::from_vv_line(line)?;
    
            if result.is_current {
                current_branch = Some(result.branch.clone())
            }
    
            branches.push(result.branch);
        }
    
        match current_branch {
            Some(current_branch) => {
                Result::Ok(
                    Repository {
                        current_branch,
                        branches,
                    }
                )
            }
            None => {
                Result::Err(
                    GitError::new_with_str("Current branch not found")
                )
            }
        }    
    }
}

#[test]
fn test_one_branch() {
    let sut = Repository::from_vv_stdout("* main 73b4084 [origin/main] commit message").unwrap();

    let current_branch = Branch::Tracked {
        name: "main".to_owned(),
        remote: RemoteBranch {
            name: "main".to_owned(),
            remote: "origin".to_owned(),
        }
    };

    let expected = Repository {
        branches: vec![current_branch.clone()],
        current_branch,
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_multiple_branches() {
    let sut = Repository::from_vv_stdout("\
        * main 73b4084 [origin/main] commit message\n\
        develop 73b4084 [origin/develop] commit message\
    ").unwrap();

    let current_branch = Branch::Tracked {
        name: "main".to_owned(),
        remote: RemoteBranch {
            name: "main".to_owned(),
            remote: "origin".to_owned(),
        },
    };

    let branches = vec![
        current_branch.clone(),
        Branch::Tracked {
            name: "develop".to_owned(),
            remote: RemoteBranch {
                name: "develop".to_owned(),
                remote: "origin".to_owned(),
            },
        },
    ];

    let expected = Repository { branches, current_branch };

    assert_eq!(sut, expected);
}

#[test]
fn test_local_branch() {
    let sut = Repository::from_vv_stdout("\
        * main 73b4084 [origin/main] commit message\n\
        local 73b4084 commit message\
    ").unwrap();

    let current_branch = Branch::Tracked {
        name: "main".to_owned(),
        remote: RemoteBranch {
            name: "main".to_owned(),
            remote: "origin".to_owned(),
        },
    };

    let branches: Vec<Branch> = vec![
        current_branch.clone(),
        Branch::Local {
            name: "local".to_owned(),
        },
    ];

    let expected = Repository { branches, current_branch };

    assert_eq!(sut, expected);
}

#[test]
fn test_dettached_branch() {
    let sut = Repository::from_vv_stdout("\
        * (HEAD detached at 1f02cc2) 1f02cc2 Initial commit\n\
        local 73b4084 commit message\
    ").unwrap();

    let current_branch = Branch::Detached;

    let branches = vec![
        current_branch.clone(),
        Branch::Local {
            name: "local".to_owned(),
        },
    ];

    let expected = Repository { branches, current_branch };

    assert_eq!(sut, expected);
}