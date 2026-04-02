use std::fmt;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug, Clone)]
pub enum CoreError {
    NotImplemented,
    InvalidConfig(String),
    InitializationFailed(String),
}

impl CoreError {
    pub fn invalid_config(message: impl Into<String>) -> Self {
        Self::InvalidConfig(message.into())
    }

    pub fn initialization_failed(message: impl Into<String>) -> Self {
        Self::InitializationFailed(message.into())
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::NotImplemented => write!(f, "not implemented"),
            CoreError::InvalidConfig(message) => write!(f, "invalid config: {message}"),
            CoreError::InitializationFailed(message) => {
                write!(f, "initialization failed: {message}")
            }
        }
    }
}

impl std::error::Error for CoreError {}
