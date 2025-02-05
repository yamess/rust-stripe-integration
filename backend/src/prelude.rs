pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Secrets not found. Cause: {0}")]
    SecretNotFound(String),
}