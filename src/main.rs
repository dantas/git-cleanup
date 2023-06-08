use git::RepositoryError;

pub mod git;

fn main() -> Result<(), git::RepositoryError> {
    // TODO: Setup integration tests to cover other possible repository layouts
 
    let repository = repository_status("/home/dantas/Documents/git-deleted-branches/test_dir/local")?;

    for branch in &repository.branches {
        if branch == &repository.current_branch {
            print!("* ")
        }
        
        match branch {
            git::Branch::Tracked { name, remote } => {
                println!("{} => [{}]", name, remote)
            }
            git::Branch::Local { name } => {
                println!("{}", name)
            }
            git:: Branch::Detached => println!("Detached")
        }
    }

    Result::Ok(())
}

fn execute_git<P, A, S>(dir: P, args: A) -> Result<String, RepositoryError>
    where P : AsRef<std::path::Path>,
          A : AsRef<[S]> + IntoIterator<Item=S>,
          S : AsRef<std::ffi::OsStr> {
    let mut command = std::process::Command::new("git");

    command.current_dir(dir);
    command.args(args);

    let output = command.output()?;

    if !output.status.success() {
        let message = match output.status.code() {
            Some(code) => {
                format!("Error executing command: {}", code)
            }
            _ => {
                "Error executing command".to_owned()
            }
        };

        return Result::Err(
            git::RepositoryError::with_message(message)
        )
    }

    let stdout = command.output()?.stdout;
    let output = String::from_utf8(stdout)?;

    Result::Ok(output)
}

// TODO: Extract execute_git to another function
fn repository_status<P : AsRef<std::path::Path>> (dir: P) -> Result<git::Repository, git::RepositoryError> {
    let output: String = execute_git(dir, ["branch", "-vv"])?;

    let results: Vec<Option<git::ParseBranchResult>> =
        output
            .lines()
            .map(git::Branch::parse_from_vv_line)
            .collect();

    let mut current_branch = None;
    let mut branches = Vec::with_capacity(results.len());

    for result in results {
        match result {
            Some(git::ParseBranchResult { branch, is_current }) => {
                if is_current {
                    current_branch = Some(branch.clone());
                }

                branches.push(branch);
            }
            None => {
                return Result::Err(git::RepositoryError::with_message("Error parsing a line".to_owned()) )
            }
        }
    }
    
    match current_branch {
        Some(branch) => {
            let repository = git::Repository {
                current_branch: branch,
                branches,
            };

            Result::Ok(repository)
        }
        None => Result::Err(
            git::RepositoryError::with_message("Current branch not found".to_owned())
        )
    }
}
