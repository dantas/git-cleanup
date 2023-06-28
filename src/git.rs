mod line;
use line::*;

mod head;
pub use head::*;

mod error;
pub use error::*;

mod repository;
pub use repository::*;

mod remote_branch;
pub use remote_branch::*;

mod branch;
pub use branch::*;

use crate::execute;

pub struct GitQuery(String);

impl GitQuery {
    pub fn query(path: &impl AsRef<std::path::Path>) -> Result<GitQuery, GitError> {
        let output = execute::execute(path, &"git", &["branch", "-vv"])?;
        Ok(GitQuery(output))
    }

    pub fn to_repository(&self) -> Result<Repository, GitError> {
        let repository = Repository::parse(&self.0)?;
        Ok(repository)
    }
}
