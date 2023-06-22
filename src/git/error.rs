use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Error compiling regex")]
    CompileRegex {
        #[from]
        source: regex::Error,
    },

    #[error("Unrecognized pattern while parsing branch, {line}")]
    BranchPattern { line: String },

    #[error("Current branch not found")]
    CurrentBranch,
}
