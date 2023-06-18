use std::env;
use git::GitError;
use args::VecArgs;

mod args;
mod git;
mod commands;
mod execute;

#[cfg(test)]
mod integration_tests;

fn main() -> Result<(), GitError> {
    let repository = git::repository(env::current_dir()?)?;

    match VecArgs::new().as_vec_str().as_slice() {
        ["list"] => commands::list(&repository),
        ["clean", args @ ..] => commands::clean(repository, args),
        _ => {
            return Err(
                GitError::new_with_str("Unrecognized command pattern")
            );
        }
    }

    Result::Ok(())
}