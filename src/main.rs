mod args;
mod git;
mod commands;
mod execute;

use std::env;
use git::GitError;
use args::VecArgs;

// TODO: Setup integration tests to cover other possible repository layouts

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