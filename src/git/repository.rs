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

    let results: Vec<Option<ParseBranchResult>> =
        output
            .lines()
            .map(Branch::parse_from_vv_line)
            .collect();

    let mut current_branch = None;
    let mut branches = Vec::with_capacity(results.len());

    for result in results {
        match result {
            Some(ParseBranchResult { branch, is_current }) => {
                if is_current {
                    current_branch = Some(branch.clone());
                }

                branches.push(branch);
            }
            None => {
                return Result::Err(RepositoryError::with_str("Error parsing a line") )
            }
        }
    }
    
    match current_branch {
        Some(branch) => {
            let repository = Repository {
                current_branch: branch,
                branches,
            };

            Result::Ok(repository)
        }
        None => Result::Err(
            RepositoryError::with_str("Current branch not found")
        )
    }
}
