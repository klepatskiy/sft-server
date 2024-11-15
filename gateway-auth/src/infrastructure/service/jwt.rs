use async_trait::async_trait;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use shaku::{Component, Interface};
use thiserror::Error;
use uuid::Uuid;
use crate::domain::jwt::jwt_model::{Claims, ClaimsTokenType};

#[async_trait]
pub trait JwtService: Interface {
    async fn create_access_token(&self, user_id: Uuid) -> Result<String, JwtServiceError>;
    async fn create_refresh_token(&self, user_id: Uuid) -> Result<String, JwtServiceError>;
    async fn verify_token(&self, token: &str, expected_type: ClaimsTokenType) -> Result<String, JwtServiceError>;
}

const REFRESH_TOKEN_DAY_DURATION: i64 = 30;
const ACCESS_TOKEN_HOUR_DURATION: i64 = 1;

#[derive(Component)]
#[shaku(interface = JwtService)]
pub struct JwtServiceImpl {
    secret_key: Vec<u8>,
    refresh_secret_key: Vec<u8>,
}

impl JwtServiceImpl {
    pub fn new(secret_key: Vec<u8>, refresh_secret_key: Vec<u8>) -> Self {
        Self { secret_key, refresh_secret_key }
    }
}

#[derive(Debug, Error)]
pub enum JwtServiceError {
    #[error("Token expire error")]
    TokenExpireError,
    #[error("Invalid token error")]
    InvalidTokenError,
    #[error("Can't create token error")]
    CreateTokenError,
    #[error("Some error")]
    SomeError,
}

#[async_trait]
impl JwtService for JwtServiceImpl {
    async fn create_access_token(&self, user_id: Uuid) -> Result<String, JwtServiceError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(ACCESS_TOKEN_HOUR_DURATION))
            .ok_or(JwtServiceError::SomeError)?
            .timestamp() as usize;

        let claims = Claims::new_access(user_id, expiration);

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.secret_key),
        ).map_err(|_| JwtServiceError::CreateTokenError)
    }

    async fn create_refresh_token(&self, user_id: Uuid) -> Result<String, JwtServiceError> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::days(REFRESH_TOKEN_DAY_DURATION))
            .ok_or(JwtServiceError::SomeError)?
            .timestamp() as usize;

        let claims = Claims::new_refresh(user_id, expiration);

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.refresh_secret_key),
        ).map_err(|_| JwtServiceError::CreateTokenError)
    }

    async fn verify_token(&self, token: &str, expected_type: ClaimsTokenType) -> Result<String, JwtServiceError> {
        let decoding_key = match expected_type {
            ClaimsTokenType::Access => &self.secret_key,
            ClaimsTokenType::Refresh => &self.refresh_secret_key,
        };

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(decoding_key),
            &Validation::default(),
        ).map_err(|_| JwtServiceError::InvalidTokenError)?;

        if token_data.claims.token_type != expected_type {
            return Err(JwtServiceError::InvalidTokenError);
        }

        if token_data.claims.exp < Utc::now().timestamp() as usize {
            return Err(JwtServiceError::InvalidTokenError);
        }

        Ok(token_data.claims.sub.clone())
    }
}
