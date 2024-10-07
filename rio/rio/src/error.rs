use thiserror::Error;

#[derive(Debug, Error)]
pub enum RioError {
    #[error("Null pointer error: [{0}]")]
    NullPointerError(String),

    #[error("IO error: [{0}]")]
    IOError(#[from] std::io::Error),
}
