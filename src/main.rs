mod args;
mod commands;
mod execute;
mod git;

#[cfg(all(test, feature = "integration"))]
mod integration_tests;

use args::VecArgs;
use std::env;
use git::GitQuery;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;

    let git_query = GitQuery::query(&path)?;
    let repository = git_query.to_repository()?;

    match VecArgs::new().to_vec_str().as_slice() {
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
