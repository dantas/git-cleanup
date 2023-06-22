use args::VecArgs;
use std::env;

mod args;
mod commands;
mod error;
mod execute;
mod git;

#[cfg(all(test, feature = "integration"))]
mod integration_tests;

use error::Error;

fn main() -> Result<(), Error> {
    let path = env::current_dir()?;
    let repository = git::query_repository(&path)?;

    match VecArgs::new().as_vec_str().as_slice() {
        ["list", args @ ..] => commands::list(&repository, args),
        ["clean", args @ ..] => commands::clean(&path, repository, args),
        ["--help"] => print_help(),
        _ => {
            println!("Unrecognized command");
            print_help();
        }
    };

    Ok(())
}

fn print_help() {
    println!("Commands available:");
    println!("    list: List branches");
    println!("    clean: Clean local branches that are not on origin");
    println!("Execute each comand with --help for available option");
}
