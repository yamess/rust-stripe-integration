pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Secrets not found. Cause: {0}")]
    SecretNotFound(String),
    #[error("Invalid Token. Cause: {0}")]
    InvalidToken(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Conversion Error. Cause: {0}")]
    ConversionError(String),
    #[error("Regex Error. Cause: {0}")]
    RegexError(#[from] regex::Error),
    #[error("Database error. Cause: {0}")]
    Database(String),
    #[error("Not found. Cause: {0}")]
    NotFound(String),
    #[error("Invalid Price. Cause: {0}")]
    InvalidPrice(String),

    #[error("Stripe Error. Cause: {0}")]
    StripeError(String),
}