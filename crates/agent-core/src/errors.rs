pub type CoreResult<T> = Result<T, CoreError>;

#[derive(Debug)]
pub enum CoreError {
    NotImplemented,
}
