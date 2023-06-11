use crate::git::Branch;
use crate::git::RepositoryError;
use crate::git::ParseBranchResult;
use crate::git::execute;

#[derive(Debug, Clone, PartialEq)]
pub struct Repository {
    pub current_branch: Branch,
    pub branches: Vec<Branch>
}

pub fn repository<P : AsRef<std::path::Path>> (dir: P) -> Result<Repository, RepositoryError> {
    let output: String = execute::git_command(dir, ["branch", "-vv"])?;

    let results: Vec<ParseBranchResult> =
        output
            .lines()
            .filter_map(Branch::from_vv_line)
            .collect();
    
    if let Some(ParseBranchResult { branch, .. }) = results.iter().find(|result| result.is_current) {
        let repository = Repository {
            current_branch: branch.clone(),
            branches: results.into_iter().map(|result| result.branch).collect(),
        };

        return Result::Ok(repository)
    };

    Result::Err(
        RepositoryError::new_with_str("Current branch not found")
    )
}
