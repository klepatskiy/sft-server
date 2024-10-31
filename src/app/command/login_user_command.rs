use async_trait::async_trait;
use std::sync::Arc;
use chrono::{Utc};
use uuid::Uuid;
use crate::app::error::AppError;
use crate::domain::user::user_entity::User;
use crate::domain::user::user_token::UserToken;

#[mockall::automock]
#[async_trait]
pub trait LoginUserRepository: Send + Sync {
    async fn get_user(&self, email: String) -> Result<User, AppError>;
}

pub trait PasswordServiceTrait: Send + Sync {
    fn verify_password(&self, hash: &str, password: &str) -> Result<(), AppError>;
}

#[async_trait]
pub trait LoginUserCommandTrait: Send + Sync {
    async fn execute(&self, email: String, password: String) -> Result<String, AppError>;
}

#[async_trait]
pub trait TokenServiceTrait: Send + Sync {
    fn generate_token(&self, user_id: Uuid) -> Result<String, AppError>;
    async fn create_user_token(&self, user_token: UserToken) -> Result<(), AppError>;
}

pub struct LoginUserCommand {
    repo: Arc<dyn LoginUserRepository>,
    password_service: Arc<dyn PasswordServiceTrait>,
    token_service: Arc<dyn TokenServiceTrait>,
}

impl LoginUserCommand {
    pub fn new(
        repo: Arc<dyn LoginUserRepository>,
        password_service: Arc<dyn PasswordServiceTrait>,
        token_service: Arc<dyn TokenServiceTrait>,
    ) -> Self {
        Self { repo, password_service, token_service }
    }
}

#[async_trait]
impl LoginUserCommandTrait for LoginUserCommand {
    async fn execute(&self, email: String, password: String) -> Result<String, AppError> {
        let user = self.repo.get_user(email).await?;
        self.password_service.verify_password(&user.password_hash, &password)?;

        let user_token = self.token_service.generate_token(user.id).map_err(|_| AppError::SomeError)?;
        let date_time = Utc::now();

        self.token_service.create_user_token(UserToken {
            id: Uuid::now_v7(),
            user_id: user.id,
            token: user_token.clone(),
            active: true,
            expires_at: date_time,
            created_at: date_time,
            updated_at: date_time,
        }).await?;

        Ok(user_token)
    }
}
