use actix_web::http::StatusCode;
use actix_web::ResponseError;

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
    #[error("Parsing Error. Cause: {0}")]
    Parsing(String),
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
    #[error("Database connection error. Cause: {0}")]
    Connection(String),

    #[error("Invalid Email. Cause: {0}")]
    InvalidEmail(String),
    #[error("Invalid role. Cause: {0}")]
    InvalidRole(String),
    #[error("Invalid user status. Cause: {0}")]
    InvalidUserStatus(String),

    #[error("Failed to authenticate user. Cause: {0}")]
    AuthenticationFailed(String),
    #[error("Firebase Error. Cause: {0}")]
    FirebaseError(String),
    #[error("Firebase Authentication Error. Cause: {0}")]
    FirebaseAuthError(String),
    #[error("Deserialization Error. Cause: {0}")]
    DeserializationError(String),

    #[error("Bad request. Cause: {0}")]
    BadRequest(String),
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid currency. Cause: {0}")]
    InvalidCurrency(String),

    #[error("Invalid billing cycle. Cause: {0}")]
    InvalidBillingCycle(String),

    #[error("Invalid subscription status. Cause: {0}")]
    InvalidSubscriptionStatus(String),

    #[error("Internal error")]
    InternalError,

    #[error("Transport error. Cause: {0}")]
    TransportError(#[from] reqwest::Error),

    #[error("External API error. Status: {0}, Body: {1}")]
    ApiError(u16, String),

    #[error("Failed to serialize data. Cause: {0}")]
    Serialization(String),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}