#[derive(Debug)]
pub enum RepositoryError {
    Source(Box<dyn std::error::Error>),
    Message(String),
}

// TODO: Is with_ the corret prefix for the below constructos?
impl RepositoryError {
    pub fn with_message(message: String) -> Self {
        RepositoryError::Message(message)
    }

    pub fn with_source(source: Box<dyn std::error::Error>) -> Self {
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

impl From<std::io::Error> for RepositoryError {
    fn from(error: std::io::Error) -> Self {
        RepositoryError::with_source(Box::new(error))
    }
}

impl From<std::string::FromUtf8Error> for RepositoryError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        RepositoryError::with_source(Box::new(error))
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