use thiserror::Error;

/// Result alias
pub type Result<T> = std::result::Result<T, GitonError>;

/// Error type for this library.
#[derive(Error, Debug)]
pub enum GitonError {
    #[error("Other: {}", &.0)]
    AdHoc(String),

    #[error("{msg}: {source:?}")]
    Compat {
        msg: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
    #[error("IO Error")]
    Io(#[from] std::io::Error),
    #[error("Config Error")]
    Config(#[from] config::ConfigError),
    #[error("Clap Error")]
    Clap(#[from] clap::Error),
    #[error("Logger Error")]
    Logger(#[from] log::SetLoggerError),
    #[error("FromUtf8Error")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("OpenAIError")]
    OpenAI(#[from] async_openai::error::OpenAIError),
    #[error("ParseIntError")]
    ParseInt(#[from] std::num::ParseIntError),
}

impl GitonError {
    /// Create a new instance of PyroscopeError
    pub fn new(msg: &str) -> Self {
        GitonError::AdHoc(msg.to_string())
    }

    /// Create a new instance of PyroscopeError with source
    pub fn new_with_source<E>(msg: &str, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        GitonError::Compat {
            msg: msg.to_string(),
            source: Box::new(source),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for GitonError {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        GitonError::AdHoc("Poison Error".to_owned())
    }
}
