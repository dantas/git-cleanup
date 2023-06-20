use std::env;
use args::VecArgs;

mod args;
mod git;
mod commands;
mod execute;
mod error;

#[cfg(test)]
mod integration_tests;

use error::Error;

fn main() -> Result<(), Error> {
    let repository = git::query_repository(env::current_dir()?)?;

    match VecArgs::new().as_vec_str().as_slice() {
        ["list"] => commands::list(&repository),
        ["clean", args @ ..] => commands::clean(repository, args),
        _ => {
            return Err(
                Error::new_with_str("Unrecognized command pattern")
            );
        }
    }

    Result::Ok(())
}