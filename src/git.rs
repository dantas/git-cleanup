use crate::execute;
use crate::error::Error;

mod repository;
pub use repository::*;

mod remote_branch;
pub use remote_branch::*;

mod branch;
pub use branch::*;

pub fn query_repository<P : AsRef<std::path::Path>> (dir: P) -> Result<Repository, Error> {
    let branch_vv_stdout: String = execute::execute(dir, "git",["branch", "-vv"])?;
    Repository::from_vv_stdout(branch_vv_stdout)
}
