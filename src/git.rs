mod error;
pub use error::GitError;

mod repository;
pub use repository::*;

mod remote_branch;
pub use remote_branch::*;

mod branch;
pub use branch::*;

use crate::execute::{self, ExecuteError};

pub struct GitOutput(String);

pub fn query(path: impl AsRef<std::path::Path>) -> Result<GitOutput, ExecuteError> {
    let output = execute::execute(&path, "git", ["branch", "-vv"])?;
    Ok(GitOutput(output))
}

pub fn repository_from(output: &GitOutput) -> Result<Repository, GitError> {
    let repository = Repository::from_vv_output(&output.0)?;
    Ok(repository)
}
