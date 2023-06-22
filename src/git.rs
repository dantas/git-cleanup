mod error;
pub use error::GitError;

mod repository;
pub use repository::*;

mod remote_branch;
pub use remote_branch::*;

mod branch;
pub use branch::*;

use crate::execute;

pub fn query_repository<P: AsRef<std::path::Path>>(
    dir: P,
) -> Result<Repository, Box<dyn std::error::Error>> {
    let branch_vv_stdout: String = execute::execute(dir, "git", ["branch", "-vv"])?;
    let repository = Repository::from_vv_stdout(branch_vv_stdout)?;
    Ok(repository)
}
