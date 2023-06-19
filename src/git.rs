mod repository;
pub use repository::*;

mod error;
pub use error::*;

mod remote_branch;
pub use remote_branch::*;

mod branch;
pub use branch::*;

use crate::execute;

pub fn query_repository<P : AsRef<std::path::Path>> (dir: P) -> Result<Repository, GitError> {
    let branch_vv_stdout: String = execute::execute(dir, "git",["branch", "-vv"])?;
    Repository::from_vv_stdout(branch_vv_stdout)
}
