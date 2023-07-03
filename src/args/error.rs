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
