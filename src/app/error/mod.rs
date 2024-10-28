use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Item not found in the database")]
    NotFound,

    #[error("Parse url error: {0}")]
    URLParseError(String),

    #[error("Invalid credentials provided")]
    InvalidCredentials,
}