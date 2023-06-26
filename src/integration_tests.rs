// I could've used the lib+bin approach, but in the end I decided to create a custom module to handle integration tests

use crate::commands;
use crate::execute;
use crate::git;
use crate::git::GitQuery;

#[test]
fn check_git_is_available() -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;
    execute::execute(&current_dir, &"git", &["--version"])?;
    Ok(())
}

#[test]
fn test_query_repository() -> Result<(), Box<dyn std::error::Error>> {
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

    let git_query = GitQuery::query(&local)?;
    let sut = git_query.to_repository()?;

    let expected = git::repository! {
        *tracked_branch { "develop", remote_branch("develop", "origin") },
        tracked_branch { "main", remote_branch("main", "origin") }
    };

    assert_eq!(sut, expected);

    Ok(())
}

#[test]
fn test_clean() -> Result<(), Box<dyn std::error::Error>> {
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
            ("git", "checkout", "develop"),
            ("git", "checkout", "-b", "local"),
            ("touch", ".localfile"),
            ("git",  "add", ".localfile"),
            ("git", "commit", "-m", "Local commit"),
            ("git", "checkout", "-b", "local_checkout")
    };

    let git_query = GitQuery::query(&local)?;
    let repository = git_query.to_repository()?;

    commands::clean(&local, repository, &[]);

    let git_query = GitQuery::query(&local)?;
    let sut = git_query.to_repository()?;

    let expected = git::repository! {
        *local_branch("local_checkout"),
        tracked_branch { "develop", remote_branch("develop", "origin") },
        tracked_branch { "main", remote_branch("main", "origin") },
    };

    assert_eq!(sut, expected);

    Ok(())
}

use rand;
use std::env;
use std::fs;
use std::path::PathBuf;

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
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
