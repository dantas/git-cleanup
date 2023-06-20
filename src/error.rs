use std::borrow::Cow;

#[derive(Debug)]
pub enum Error {
    Source(Box<dyn std::error::Error>),
    Message(Cow<'static, str>),
}

// Most of this code can be automated by the crate thiserror

impl Error {
    pub fn new_with_str(message: &'static str) -> Self {
        Error::Message(Cow::Borrowed(message))
    }

    pub fn new_with_string(message: String) -> Self {
        Error::Message(Cow::Owned(message))
    }

    pub fn new_with_source(source: Box<dyn std::error::Error>) -> Self {
        Error::Source(source)
    }
}

macro_rules! new_error_with_string {
    ($($arg:tt)*) => {
        Error::new_with_string(format!($($arg)*))
    }
}

pub(crate) use new_error_with_string;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Message(message) => {
                write!(f, "RepositoryError, message: {}", message)?;
            }
            Error::Source(_) => {
                write!(f, "RepositoryError, caused by source error")?;
            }
        };

        std::fmt::Result::Ok(())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Message(_) => {
                None
            }
            Error::Source(box_source) => {
                Some(box_source.as_ref())
            }
        }
    }
}