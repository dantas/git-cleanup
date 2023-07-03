mod arguments;
mod error;
mod parser;
mod parser_tests;

pub use arguments::*;
use error::ParseError;
use parser::ArgumentsParser;
use std::env;

pub fn parse_arguments() -> Result<Arguments, ParseError> {
    ArgumentsParser::new(env::args().skip(1)).parse()
}
