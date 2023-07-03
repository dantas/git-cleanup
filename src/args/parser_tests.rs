#![cfg(test)]

use super::arguments::*;
use super::error::ParseError;
use super::ArgumentsParser;
use std::collections::HashSet;
use std::path::PathBuf;

#[test]
fn help_only() {
    let sut = parse(&["--help"]).unwrap();

    let mut options = HashSet::new();
    options.insert(ProgramOption::Help);

    let expected = Arguments {
        options,
        command: None,
    };

    assert_eq!(sut, expected);
}

#[test]
fn command_without_option() {
    let sut = parse(&["list"]).unwrap();

    let expected = Arguments {
        options: HashSet::new(),
        command: Some(ProgramCommand::List(ListOption::Gone)),
    };

    assert_eq!(sut, expected);
}

#[test]
fn command_with_option() {
    let sut = parse(&["list", "--all"]).unwrap();

    let expected = Arguments {
        options: HashSet::new(),
        command: Some(ProgramCommand::List(ListOption::All)),
    };

    assert_eq!(sut, expected);
}

#[test]
fn multiple_options_for_command() {
    let sut = parse(&["list", "--all", "--gone"]).unwrap();

    let expected = Arguments {
        options: HashSet::new(),
        command: Some(ProgramCommand::List(ListOption::All)),
    };

    assert_eq!(sut, expected);
}

#[test]
fn multiple_options_and_command_with_option() {
    let sut = parse(&["--path", "/", "--fetch-prune", "clean", "--automatic"]).unwrap();

    let mut options = HashSet::new();
    options.insert(ProgramOption::FetchPrune);
    options.insert(ProgramOption::Path(PathBuf::from("/")));

    let expected = Arguments {
        options,
        command: Some(ProgramCommand::Clean(CleanOption::Automatic)),
    };

    assert_eq!(sut, expected);
}

#[test]
fn path_missing() {
    let sut = parse(&["--path", "clean"]).err().unwrap();

    let expected = ParseError::OptionPath;

    assert_eq!(sut, expected)
}

#[test]
fn path_missing_2() {
    let sut = parse(&["--path", "--fetch--prune", "clean"]).err().unwrap();

    let expected = ParseError::OptionPath;

    assert_eq!(sut, expected)
}

#[test]
fn path_not_provided() {
    let sut = parse(&["--path"]).err().unwrap();

    let expected = ParseError::OptionPath;

    assert_eq!(sut, expected)
}

#[test]
fn path_doesnt_exist() {
    let sut = parse(&["--path", "/something/not/exist", "list"])
        .err()
        .unwrap();

    let expected = ParseError::OptionPath;

    assert_eq!(sut, expected)
}

fn parse(token_source: &[&str]) -> Result<Arguments, ParseError> {
    ArgumentsParser::new(token_source.iter()).parse()
}
