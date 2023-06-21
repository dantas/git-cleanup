use args::VecArgs;
use std::env;

mod args;
mod commands;
mod error;
mod execute;
mod git;

#[cfg(test)]
mod integration_tests;

use error::Error;

fn main() -> Result<(), Error> {
    let path = env::current_dir()?;
    let repository = git::query_repository(&path)?;

    let success = match VecArgs::new().as_vec_str().as_slice() {
        ["list", args @ ..] => commands::list(&repository, args),
        ["clean", args @ ..] => commands::clean(&path, repository, args),
        _ => {
            return Err(Error::new_with_str("Unrecognized command pattern"));
        }
    };

    if !success {
        commands::print_list_help();
    }

    Result::Ok(())
}
