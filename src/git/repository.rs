use crate::git::Branch;
use crate::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Repository {
    pub current_branch: Branch,
    pub branches: Vec<Branch>
}

impl Repository {
    pub(super) fn from_vv_stdout<S : AsRef<str>>(command_stdout: S) -> Result<Repository, Error> {
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
                    Error::new_with_str("Current branch not found")
                )
            }
        }    
    }
}

#[test]
fn test_one_branch() {
    let sut = Repository::from_vv_stdout("* main 73b4084 [origin/main] commit message").unwrap();

    let expected = repository! {
        *tracked_branch { "main", remote_branch("main", "origin") }
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_multiple_branches() {
    let sut = Repository::from_vv_stdout("\
        * main 73b4084 [origin/main] commit message\n\
        develop 73b4084 [origin/develop] commit message\
    ").unwrap();

    let expected = repository! {
        *tracked_branch { "main" , remote_branch("main", "origin") },
        tracked_branch { "develop" , remote_branch("develop", "origin") },
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_local_branch() {
    let sut = Repository::from_vv_stdout("\
        * main 73b4084 [origin/main] commit message\n\
        local 73b4084 commit message\
    ").unwrap();

    let expected = repository! {
        *tracked_branch { "main", remote_branch("main", "origin") },
        local_branch("local"),
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_dettached_branch() {
    let sut = Repository::from_vv_stdout("\
        * (HEAD detached at 1f02cc2) 1f02cc2 Initial commit\n\
        local 73b4084 commit message\
    ").unwrap();

    let expected = repository! {
        *detached,
        local_branch("local"),
    };

    assert_eq!(sut, expected);
}

#[cfg(test)]
macro_rules! repository {
    ( * $type:ident $args:tt $( , $rest_type:ident $rest_args:tt ),* $(,)? ) => { 
        {
            let current_branch = $crate::git::make_branch!($type $args);

            let branches = vec![
                current_branch.clone(),
                $(
                    $crate::git::make_branch!{ $rest_type $rest_args }
                ),*
            ];

            Repository {
                current_branch,
                branches,
            }
        }
    };

    ( * $type:ident $( , $rest_type:ident $rest_args:tt )* $(,)? ) => { 
        {
            let current_branch = $crate::git::make_branch!($type);

            let branches = vec![
                current_branch.clone(),
                $(
                    $crate::git::make_branch!{ $rest_type $rest_args }
                ),*
            ];

            Repository {
                current_branch,
                branches,
            }
        }
    };
}

#[cfg(test)]
pub(crate) use repository;