use crate::execute;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Error parsing git query: {source}")]
    Parse {
        #[from]
        source: GitParseError,
    },

    #[error("Error querying git: {source}")]
    Query {
        #[from]
        source: execute::ExecuteError,
    },
}

#[derive(Error, Debug)]
pub enum GitParseError {
    #[error("Unrecognized pattern while parsing branch, {line}")]
    BranchPattern { line: String },

    #[error("Current branch not found")]
    CurrentBranch,
}
