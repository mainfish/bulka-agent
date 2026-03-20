use std::fmt;

pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug)]
pub enum CoreError {
    NotImplemented,
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::NotImplemented => write!(f, "not implemented"),
        }
    }
}

impl std::error::Error for CoreError {}
