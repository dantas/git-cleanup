use args::VecArgs;
use std::env;

mod args;
mod commands;
mod execute;
mod git;

#[cfg(all(test, feature = "integration"))]
mod integration_tests;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;

    let git_output = git::query_git(&path)?;
    let repository = git::repository_from(&git_output)?;

    match VecArgs::new().vec_str().as_slice() {
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
