mod line_parser;
use line_parser::*;

mod head;
pub use head::*;

mod error;
pub use error::*;

mod repository;
pub use repository::*;

mod remote_branch;
pub use remote_branch::*;

mod remote_branch_status;
pub use remote_branch_status::*;

mod branch;
pub use branch::*;

use crate::execute;

/*
    This struct stores the output of git branch -vv
    Repository and its underlying data structures point to
    pieces of this string, preventing the allocation of many
    Strings
*/
pub struct GitQuery(String);

impl GitQuery {
    fn lines(&self) -> impl Iterator<Item = impl LineParser> {
        self.0.lines().map(new_line_parser)
    }

    fn count_lines(&self) -> usize {
        self.0.lines().count()
    }
}

impl GitQuery {
    pub fn query(path: &impl AsRef<std::path::Path>) -> Result<GitQuery, GitError> {
        let output = execute::execute(path, "git", &["branch", "-vv"])?;
        Ok(GitQuery(output))
    }

    pub fn to_repository(&self) -> Result<Repository, GitError> {
        let repository = Repository::parse(self)?;
        Ok(repository)
    }
}
