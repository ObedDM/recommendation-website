use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignupAuthError {
    #[error("Failed to hash password: {0}")]
    PasswordHashingError(String),

    #[error("Failed to insert user into database: {0}")]
    DatabaseCreateUserError(String),

    #[error("Failed to create user. Username already exists: {0}")]
    UsernameAlreadyExists(String),

    #[error("Failed to create user. Email already registered: {0}")]
    EmailAlreadyExists(String),

    #[error("Failed to delete newly-created user as a rollback from creating profile picture error: {0}")]
    UserCreationRollbackFailed(String),

    #[error("Failed to create profile picture for newly-created user: {0}")]
    ProfilePictureCreationFailed(String),
}

#[derive(Error, Debug)]
pub enum LoginAuthError {
    #[error("Password does not match")]
    PasswordMatchError,

    #[error("Failed to verify password: {0}")]
    PasswordVerificationError(String),

    #[error("Failed to verify user credentials: {0}")]
    DatabaseVerifyUserError(String),

    #[error("No password found for user. User might not exist")]
    UserNotFound,
}