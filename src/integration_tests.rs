#![cfg(feature = "integration")]

// I could've used the lib+bin approach, but in the end I decided to create a custom module to handle integration tests

use crate::error::Error;
use crate::execute;
use crate::git;
use crate::git::Repository;

#[test]
fn test_query_repository() -> Result<(), Error> {
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

    let sut = git::query_repository(&local)?;

    let expected = git::repository! {
        *tracked_branch { "develop", remote_branch("develop", "origin") },
        tracked_branch { "main", remote_branch("main", "origin") }
    };

    assert_eq!(sut, expected);

    Ok(())
}

// #[test]
// fn test_

use rand;
use std::env;
use std::fs;
use std::path::PathBuf;

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new() -> Result<Self, Error> {
        let random_dir_name = rand::random::<u32>().to_string();
        let path = env::temp_dir().join(random_dir_name);
        fs::create_dir(path.clone())?;
        Ok(TempDir { path })
    }

    fn join<P: AsRef<std::path::Path>>(&self, path: P) -> TempDir {
        TempDir {
            path: self.path.join(path.as_ref()),
        }
    }
}

impl AsRef<std::path::Path> for TempDir {
    fn as_ref(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir(self);
    }
}
