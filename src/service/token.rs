use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use async_trait::async_trait;
use thiserror::Error;
use chrono::{Utc, Duration};
use uuid::Uuid;
use crate::app::command::login_user_command::{TokenServiceTrait};
use crate::app::error::AppError;
use crate::domain::user::user_token::{UserToken};
use crate::repository::postgres::user::user_token_repository::PostgresUserTokenRepository;

#[derive(Debug, Error)]
pub enum TokenServiceError {
    #[error("Token creation error")]
    CreationError,
    #[error("Token verification error")]
    VerificationError,
    #[error("User not found error")]
    UserNotError,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct TokenService {
    secret: Arc<String>,
    postgres_user_token_repository: Arc<PostgresUserTokenRepository>,
}

#[async_trait]
impl TokenServiceTrait for TokenService {
    fn generate_token(&self, user_id: Uuid) -> Result<String, AppError> {
        self.generate_token(user_id).map_err(|_| AppError::InvalidCredentials)
    }

    async fn create_user_token(&self, user_token: UserToken) -> Result<(), AppError> {
        self.postgres_user_token_repository.create_user_token(user_token).await
    }
}

impl TokenService {
    pub fn new(
        secret: String,
        postgres_user_token_repository:
        Arc<PostgresUserTokenRepository>,
    ) -> Self {
        Self {
            secret: Arc::new(secret),
            postgres_user_token_repository,
        }
    }

    pub fn generate_token(&self, user_id: Uuid) -> Result<String, TokenServiceError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(365))
            .expect("valid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
        };

        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_bytes()))
            .map_err(|_| TokenServiceError::CreationError)
    }

    pub async fn get_user_from_token(&self, token: &str) -> Result<(), TokenServiceError> {
        let user_with_token = self.
            postgres_user_token_repository.
            get_user_token(token).
            await.
            map_err(|_| TokenServiceError::UserNotError)?;
        self.validate_token(user_with_token.user_token)?;

        Ok(())
    }

    fn validate_token(&self, user_token: UserToken) -> Result<(), TokenServiceError> {
        let expiration = Utc::now();

        if user_token.expires_at < expiration {
            return Err(TokenServiceError::VerificationError);
        }

        Ok(())
    }
}