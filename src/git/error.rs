use std::borrow::Cow;

#[derive(Debug)]
pub enum RepositoryError {
    Source(Box<dyn std::error::Error>),
    Message(Cow<'static, str>),
}

// Most of this code can be automated by the crate thiserror

impl RepositoryError{
    pub fn new_with_str(message: &'static str) -> Self {
        RepositoryError::Message(Cow::Borrowed(message))
    }

    pub fn new_with_string(message: String) -> Self {
        RepositoryError::Message(Cow::Owned(message))
    }

    pub fn new_with_source(source: Box<dyn std::error::Error>) -> Self {
        RepositoryError::Source(source)
    }
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::Message(message) => {
                write!(f, "RepositoryError, message: {}", message)?;
            }
            RepositoryError::Source(_) => {
                write!(f, "RepositoryError, caused by source error")?;
            }
        };

        std::fmt::Result::Ok(())
    }
}

impl std::error::Error for RepositoryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RepositoryError::Message(_) => {
                None
            }
            RepositoryError::Source(box_source) => {
                Some(box_source.as_ref())
            }
        }
    }
}