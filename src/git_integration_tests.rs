#![cfg(all(test, feature = "testbin"))]
use crate::args::CleanOption;
use crate::commands;
use crate::execute;
use crate::git;
use crate::git::GitQuery;
use crate::test_support::TempDir;
use std::env;

#[test]
fn check_git_is_available() {
    let current_dir = env::current_dir().unwrap();
    execute::execute(&current_dir, "git", &["--version"]).unwrap();
}

#[test]
fn test_query_repository() {
    let root = TempDir::new().unwrap();
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

    let git_query = GitQuery::query(&local).unwrap();
    let sut = git_query.to_repository().unwrap();

    let expected = git::repository! {
        *tracking { "develop", remote("develop", "origin", synchronized) },
        tracking { "main", remote("main", "origin", synchronized) }
    };

    assert_eq!(sut, expected);
}

#[test]
fn test_clean() {
    let root = TempDir::new().unwrap();
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
            ("git", "checkout", "-b", "feature_branch"),
            ("git", "checkout", "main")

        root:
            ("git", "clone", "-l", "remote", "local")

        local:
            ("git", "checkout", "main"),
            ("git", "checkout", "develop"),
            ("git", "checkout", "feature_branch"),
            ("git", "checkout", "-b", "local_checkout"),
            ("touch", ".localfile"),
            ("git",  "add", ".localfile"),
            ("git", "commit", "-m", "Local commit"),
            ("git", "checkout", "develop")

        remote:
            ("git", "branch", "-D", "feature_branch")

        local:
            ("git", "fetch", "--prune")
    };

    let git_query = GitQuery::query(&local).unwrap();
    let repository = git_query.to_repository().unwrap();

    commands::clean(local.as_ref(), repository, &CleanOption::Automatic);

    let git_query = GitQuery::query(&local).unwrap();
    let sut = git_query.to_repository().unwrap();

    let expected = git::repository! {
        *tracking { "develop", remote("develop", "origin", synchronized) },
        local("local_checkout"),
        tracking { "main", remote("main", "origin", synchronized) },
    };

    assert_eq!(sut, expected);
}
