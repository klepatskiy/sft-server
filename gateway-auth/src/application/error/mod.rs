use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Token expire error")]
    TokenExpireError,
    #[error("Invalid token error")]
    InvalidTokenError,
    #[error("Can't create token error")]
    CreateTokenError,
    #[error("Some error")]
    SomeError,
}