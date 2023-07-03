mod arguments;
mod error;
mod parser;
mod parser_tests;

use arguments::Arguments;
use error::ParseError;
use parser::ArgumentsParser;
use std::env;

pub struct VecArgs(Vec<String>);

impl VecArgs {
    pub fn new() -> Self {
        VecArgs(Vec::from_iter(env::args().skip(1)))
    }

    pub fn to_vec_str(&self) -> Vec<&str> {
        Vec::from_iter(self.0.iter().map(|s| s.as_str()))
    }
}

pub fn parse_arguments() -> Result<Arguments, ParseError> {
    let mut parser = ArgumentsParser::new(std::env::args().skip(1));
    parser.parse()
}
