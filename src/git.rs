mod error;
pub use error::GitError;

mod repository;
pub use repository::*;

mod remote_branch;
pub use remote_branch::*;

mod branch;
pub use branch::*;

use crate::execute::{self, ExecuteError};

pub struct GitQuery(String);

pub fn query_git(path: impl AsRef<std::path::Path>) -> Result<GitQuery, ExecuteError> {
    let output = execute::execute(&path, "git", ["branch", "-vv"])?;
    Ok(GitQuery(output))
}

pub fn repository_from(query: &GitQuery) -> Result<Repository, GitError> {
    let repository = Repository::from_vv_output(&query.0)?;
    Ok(repository)
}
