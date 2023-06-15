use std::borrow::Cow;

#[derive(Debug)]
pub enum GitError {
    Source(Box<dyn std::error::Error>),
    Message(Cow<'static, str>),
}

// Most of this code can be automated by the crate thiserror

impl GitError {
    pub fn new_with_str(message: &'static str) -> Self {
        GitError::Message(Cow::Borrowed(message))
    }

    pub fn new_with_string(message: String) -> Self {
        GitError::Message(Cow::Owned(message))
    }

    pub fn new_with_source(source: Box<dyn std::error::Error>) -> Self {
        GitError::Source(source)
    }
}

macro_rules! new_git_error_with_string {
    ($($arg:tt)*) => {
        GitError::new_with_string(format!($($arg)*))
    }
}

pub(super) use new_git_error_with_string;

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::Message(message) => {
                write!(f, "RepositoryError, message: {}", message)?;
            }
            GitError::Source(_) => {
                write!(f, "RepositoryError, caused by source error")?;
            }
        };

        std::fmt::Result::Ok(())
    }
}

impl std::error::Error for GitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GitError::Message(_) => {
                None
            }
            GitError::Source(box_source) => {
                Some(box_source.as_ref())
            }
        }
    }
}