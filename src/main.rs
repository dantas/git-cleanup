mod args;
mod commands;
mod execute;
mod git;
mod git_integration_tests;
mod test_support;

use args::{Arguments, ProgramCommand};
use execute::ExecuteError;
use git::GitQuery;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = args::parse_arguments()?;

    if print_help(&arguments) {
        return Ok(());
    }

    let path = path_from(&arguments)?;

    fetch_prune(&path, &arguments)?;

    let git_query = GitQuery::query(&path)?;
    let repository = git_query.to_repository()?;

    match arguments.command {
        Some(ProgramCommand::List(option)) => commands::list(&repository, &option),
        Some(ProgramCommand::Clean(option)) => commands::clean(&path, repository, &option),
        _ => {}
    };

    Ok(())
}

fn print_help(arguments: &Arguments) -> bool {
    let print_help = arguments.options.is_empty() && arguments.command.is_none()
        || arguments.options.contains(&args::ProgramOption::Help);

    if print_help {
        println!("Available commands:");
        println!("    list: List branches");
        println!("    clean: Delete local branches that are gone from origin");
        println!("Execute each comand with --help for available options");
    }

    print_help
}

fn path_from(arguments: &Arguments) -> Result<PathBuf, std::io::Error> {
    let mut path = env::current_dir()?;

    for option in &arguments.options {
        match option {
            args::ProgramOption::Path(custom_path) => {
                path = custom_path.clone();
            }
            _ => continue,
        }
    }

    Ok(path)
}

fn fetch_prune(path: &PathBuf, arguments: &Arguments) -> Result<(), ExecuteError> {
    if arguments.options.contains(&args::ProgramOption::FetchPrune) {
        let _ = execute::execute(path, &"git", &["fetch", "--prune"])?;
    }

    Ok(())
}
