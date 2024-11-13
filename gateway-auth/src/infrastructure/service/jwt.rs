use async_trait::async_trait;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, TokenData};
use chrono::{Utc, Duration};
use shaku::{Component, Interface};
use crate::domain::jwt::jwt_model::{Claims, RefreshClaims};

#[async_trait]
pub trait JwtService: Interface {
    async fn create_jwt(&self, user_id: &str) -> Result<(String, String), jsonwebtoken::errors::Error>;
    async fn verify_jwt(&self, token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error>;
    async fn verify_refresh_token(&self, token: &str) -> Result<TokenData<RefreshClaims>, jsonwebtoken::errors::Error>;
}

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

#[async_trait]
impl JwtService for JwtServiceImpl {
    async fn create_jwt(&self, user_id: &str) -> Result<(String, String), jsonwebtoken::errors::Error> {
        let access_expiration = Utc::now().checked_add_signed(Duration::hours(1)).expect("Error with expiration").timestamp() as usize;
        let claims = Claims { sub: user_id.to_owned(), exp: access_expiration };
        let access_token = encode(&Header::default(), &claims, &EncodingKey::from_secret(&self.secret_key))?;
        
        let refresh_expiration = Utc::now().checked_add_signed(Duration::days(7)).expect("Error with expiration").timestamp() as usize;
        let refresh_claims = RefreshClaims { sub: user_id.to_owned(), exp: refresh_expiration };
        let refresh_token = encode(&Header::default(), &refresh_claims, &EncodingKey::from_secret(&self.refresh_secret_key))?;
        
        Ok((access_token, refresh_token))
    }

    async fn verify_jwt(&self, token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(token, &DecodingKey::from_secret(&self.secret_key), &Validation::default())
    }

    async fn verify_refresh_token(&self, token: &str) -> Result<TokenData<RefreshClaims>, jsonwebtoken::errors::Error> {
        decode::<RefreshClaims>(token, &DecodingKey::from_secret(&self.refresh_secret_key), &Validation::default())
    }
}
