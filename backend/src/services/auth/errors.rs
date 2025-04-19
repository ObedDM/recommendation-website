use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Failed to hash password: {0}")]
    PasswordHashingError(String),

    #[error("Failed to insert user into database: {0}")]
    DatabaseCreateUserError(String),
}