use std::collections::HashSet;
use std::env;
use std::iter::Peekable;
use std::path::PathBuf;

pub fn parse_arguments() -> Result<Arguments, ParseError> {
    ArgumentsParser::new(env::args().skip(1)).parse()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Arguments {
    pub options: HashSet<ProgramOption>,
    pub command: Option<ProgramCommand>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ProgramOption {
    Help,
    Path(PathBuf),
    FetchPrune,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramCommand {
    List(ListOption),
    Clean(CleanOption),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListOption {
    Help,
    All,
    Local,
    Tracked,
    Gone,
    Diverged,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CleanOption {
    Help,
    Step,
    Automatic,
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("Error parsing option --path")]
    OptionPath,

    #[error("Command not found")]
    Command,

    #[error("Error parsing list options")]
    ListOption,

    #[error("Error parsing clean options")]
    CleanOption,
}

struct ArgumentsParser<I, S>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    pub token_source: Peekable<I>,
}

impl<I, S> ArgumentsParser<I, S>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    fn new(iterator: I) -> Self {
        ArgumentsParser {
            token_source: iterator.peekable(),
        }
    }

    fn parse(&mut self) -> Result<Arguments, ParseError> {
        let mut options = HashSet::<ProgramOption>::new();

        let mut parsed_program_option = self.parse_program_option()?;

        while let Some(program_option) = parsed_program_option {
            options.insert(program_option);
            parsed_program_option = self.parse_program_option()?;
        }

        let command = self.parse_command()?;

        Ok(Arguments { options, command })
    }

    fn parse_program_option(&mut self) -> Result<Option<ProgramOption>, ParseError> {
        let token = match self.token_source.peek() {
            Some(token) => token,
            None => return Ok(None),
        };

        match token.as_ref() {
            "--help" => {
                let _ = self.token_source.next();
                Ok(Some(ProgramOption::Help))
            }
            "--path" => {
                let _ = self.token_source.next();
                let pathbuf = self.parse_validate_path_option()?;
                Ok(Some(ProgramOption::Path(pathbuf)))
            }
            "--fetch-prune" => {
                let _ = self.token_source.next();
                Ok(Some(ProgramOption::FetchPrune))
            }
            _ => Ok(None),
        }
    }

    fn parse_validate_path_option(&mut self) -> Result<PathBuf, ParseError> {
        let token_path = match self.token_source.next() {
            Some(token_path) => token_path,
            None => return Err(ParseError::OptionPath),
        };

        let path = match PathBuf::try_from(token_path.as_ref()) {
            Ok(path) => path,
            Err(_) => return Err(ParseError::OptionPath),
        };

        /*
           Check if path exists
           This check prevents the code from wrongly interpreting other tokens as valid paths
           It also ensures that we can give the user a more meaningful error message instead of
           letting the code fail in other layers, which may lead to criptic error messages
        */
        match path.try_exists() {
            Ok(exists) if exists => Ok(path),
            _ => Err(ParseError::OptionPath),
        }
    }

    fn parse_command(&mut self) -> Result<Option<ProgramCommand>, ParseError> {
        let token = match self.token_source.next() {
            Some(token) => token,
            None => return Ok(None),
        };

        let command = match token.as_ref() {
            "list" => ProgramCommand::List(self.parse_list_options()?),
            "clean" => ProgramCommand::Clean(self.parse_clean_options()?),
            _ => return Err(ParseError::Command),
        };

        Ok(Some(command))
    }

    fn parse_list_options(&mut self) -> Result<ListOption, ParseError> {
        let token = match self.token_source.next() {
            Some(token) => token,
            None => return Ok(ListOption::Gone),
        };

        match token.as_ref() {
            "--help" => Ok(ListOption::Help),
            "--all" => Ok(ListOption::All),
            "--tracked" => Ok(ListOption::Tracked),
            "--local" => Ok(ListOption::Local),
            "--gone" => Ok(ListOption::Gone),
            "--diverged" => Ok(ListOption::Diverged),
            _ => Err(ParseError::ListOption),
        }
    }

    fn parse_clean_options(&mut self) -> Result<CleanOption, ParseError> {
        let token = match self.token_source.next() {
            Some(token) => token,
            None => return Ok(CleanOption::Step),
        };

        match token.as_ref() {
            "--help" => Ok(CleanOption::Help),
            "--step" => Ok(CleanOption::Step),
            "--automatic" => Ok(CleanOption::Automatic),
            _ => Err(ParseError::CleanOption),
        }
    }
}

#[test]
fn help_only() {
    let sut = parse_in_test(&["--help"]).unwrap();

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
    let sut = parse_in_test(&["list"]).unwrap();

    let expected = Arguments {
        options: HashSet::new(),
        command: Some(ProgramCommand::List(ListOption::Gone)),
    };

    assert_eq!(sut, expected);
}

#[test]
fn command_with_option() {
    let sut = parse_in_test(&["list", "--all"]).unwrap();

    let expected = Arguments {
        options: HashSet::new(),
        command: Some(ProgramCommand::List(ListOption::All)),
    };

    assert_eq!(sut, expected);
}

#[test]
fn multiple_options_for_command() {
    let sut = parse_in_test(&["list", "--all", "--gone"]).unwrap();

    let expected = Arguments {
        options: HashSet::new(),
        command: Some(ProgramCommand::List(ListOption::All)),
    };

    assert_eq!(sut, expected);
}

#[test]
fn multiple_options_and_command_with_option() {
    let sut = parse_in_test(&["--path", "/", "--fetch-prune", "clean", "--automatic"]).unwrap();

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
    let sut = parse_in_test(&["--path", "clean"]).err().unwrap();

    let expected = ParseError::OptionPath;

    assert_eq!(sut, expected)
}

#[test]
fn path_missing_2() {
    let sut = parse_in_test(&["--path", "--fetch--prune", "clean"])
        .err()
        .unwrap();

    let expected = ParseError::OptionPath;

    assert_eq!(sut, expected)
}

#[test]
fn path_not_provided() {
    let sut = parse_in_test(&["--path"]).err().unwrap();

    let expected = ParseError::OptionPath;

    assert_eq!(sut, expected)
}

#[test]
fn path_doesnt_exist() {
    let sut = parse_in_test(&["--path", "/something/not/exist", "list"])
        .err()
        .unwrap();

    let expected = ParseError::OptionPath;

    assert_eq!(sut, expected)
}

#[cfg(test)]
fn parse_in_test(token_source: &[&str]) -> Result<Arguments, ParseError> {
    ArgumentsParser::new(token_source.iter()).parse()
}
