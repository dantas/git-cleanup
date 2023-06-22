mod error;
pub use error::GitError;

mod repository;
pub use repository::*;

mod remote_branch;
pub use remote_branch::*;

mod branch;
pub use branch::*;

use crate::execute;

pub fn query_repository<P>(path: P) -> Result<Repository, Box<dyn std::error::Error>>
where
    P: AsRef<std::path::Path>,
{
    let branch_vv_stdout: String = execute::execute(path, "git", ["branch", "-vv"])?;
    let repository = Repository::from_vv_stdout(branch_vv_stdout)?;
    Ok(repository)
}