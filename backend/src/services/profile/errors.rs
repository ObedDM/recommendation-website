use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProfileDeleteError {
    #[error("Failed to delete user ({0}): {1}")]
    UserDeletionFailed(String, String),

    #[error("Failed to delete user ({0}): user was not found")]
    UserNotFound(String),
}

#[derive(Error, Debug)]

pub enum ProfilePictureError {
    #[error("Failed to open file {0}: {1}; at {2}. File might not exist")]
    FileReadingFailed(String, String, String),

    #[error("Failed to save image {0}: {1}; at {2}")]
    ImageSavingFailed(String, String, String),

    #[error("Failed to decode image ({0}): {1}")]
    ImageDecodingFailed(String, String),

    #[error("Error when trying to update file {0}: {1}; at {2}")]
    ErrorUpdatingFile(String, String, String),

    #[error("Multipart parsing failed: {0}")]
    MultipartError(String),

    #[error("Missing field `{0}` in multipart data")]
    MissingField(String),

    #[error("Failed to read bytes: {0}")]
    ByteReadError(String),
}