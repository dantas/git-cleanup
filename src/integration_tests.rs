// I could've used the lib+bin approach, but in the end I decided to create a custom module to handle integration tests

use crate::git;
use crate::execute;
use crate::git::Branch;
use crate::git::RemoteBranch;
use crate::git::Repository;

#[test]
fn test_standard_repository() -> Result<(), git::GitError> {
    let root = TempDir::new()?;
    let remote = root.join("remote");
    let local = root.join("local");

    execute::sequence_execute! {
        root:
            ("mkdir", "remote")

        remote:
            ("git", "init", "-b", "main"),
            ("touch", ".mainfile"),
            ("git", "add", ".mainfile"),
            ("git", "commit", "-m", "Main commit"),
            ("git", "checkout", "-b", "develop"),
            ("touch", ".developfile"),
            ("git", "add", ".developfile"),
            ("git", "commit", "-m", "Develop commit"),
            ("git", "checkout", "main")

        root:
            ("git", "clone", "-l", "remote", "local")

        local:
            ("git", "checkout", "main"),
            ("git", "checkout", "develop")
    };


    let sut = git::repository(&local).expect("Error extracting repository information");

    let current_branch = Branch::Tracked {
        name: "develop".to_owned(),
        remote: RemoteBranch {
            name: "develop".to_owned(),
            remote: "origin".to_owned(),
        }
    };

    let expected = Repository {
        current_branch: current_branch.clone(),
        branches: vec![
            current_branch,
            Branch::Tracked {
                name: "main".to_owned(),
                remote: RemoteBranch {
                    name: "main".to_owned(),
                    remote: "origin".to_owned(),
                }
            }
        ]
    };

    assert_eq!(sut, expected);

    Result::Ok(())
}

use rand;
use std::env;
use std::fs;
use std::path::PathBuf;

struct TempDir {
    path: PathBuf
}

impl TempDir {
    fn new() -> Result<Self, git::GitError> {
        let random_dir_name = rand::random::<u32>().to_string();
        let path = env::temp_dir().join(random_dir_name);
        fs::create_dir(path.clone())?;
        Result::Ok(
            TempDir { path }
        )
    }

    pub fn join<P: AsRef<std::path::Path>>(&self, path: P) -> TempDir {
        TempDir {
            path: self.path.join(path.as_ref())
        }
    }
}

impl AsRef<std::path::Path> for TempDir {
    fn as_ref(&self) -> &std::path::Path {
        return &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        fs::remove_dir(self);
    }
}