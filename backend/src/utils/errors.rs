use thiserror::Error;

#[derive(Error, Debug)]
pub enum JWTError {
    #[error("Authentication failed: missing JWT Token: ({0})")]
    MissingToken(String),

    #[error("JWT decoding failed: ({0})")]
    DecodingTokenError(String),

    #[error("JWT encoding failed: ({0})")]
    EncodingTokenError(String),

    #[error("Failed to verify user in database: {0}")]
    DatabaseVerifyUserError(String),

    #[error("User doesnt exist.")]
    UserNotFound,

    #[error("Failed to authenticate user: ({0})")]
    UserAuthFailed(String),

    #[error("Token has expired")]
    ExpiredToken,
}