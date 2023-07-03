use super::arguments::*;
use super::error::ParseError;
use std::collections::HashSet;
use std::iter::Peekable;
use std::path::PathBuf;

pub(super) struct ArgumentsParser<I, S>
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
    pub fn new(iterator: I) -> Self {
        ArgumentsParser {
            token_source: iterator.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Arguments, ParseError> {
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
            "--step" => Ok(CleanOption::Step),
            "--automatic" => Ok(CleanOption::Automatic),
            _ => Err(ParseError::CleanOption),
        }
    }
}
