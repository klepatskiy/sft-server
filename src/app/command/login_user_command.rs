use crate::app::error::AppError;
use async_trait::async_trait;
use crate::domain::user::user_entity::User;

#[mockall::automock]
#[async_trait]
pub trait LoginUserRepository {
    async fn get_user(&self, email: String) -> Result<User, AppError>;
}

pub trait PasswordServiceTrait: Send + Sync {
    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, AppError>;
}

#[async_trait]
pub trait LoginUserCommandTrait: Send + Sync {
    async fn execute(&self, email: String, password: String) -> Result<String, AppError>;
}

pub struct LoginUserCommand<R, S>
where
    R: LoginUserRepository,
    S: PasswordServiceTrait,
{
    repo: R,
    password_service: S,
}

impl<R, S> LoginUserCommand<R, S>
where
    R: LoginUserRepository,
    S: PasswordServiceTrait + Send + Sync + 'static,
{
    pub fn new(repo: R, password_service: S) -> Self {
        Self { repo, password_service }
    }
}

#[async_trait]
impl<R, S> LoginUserCommandTrait for LoginUserCommand<R, S>
where
    R: LoginUserRepository + Send + Sync + 'static,
    S: PasswordServiceTrait + Send + Sync + 'static,
{
    async fn execute(&self, email: String, password: String) -> Result<String, AppError> {
        let user = self.repo.get_user(email).await?;
        // Логика проверки пароля и возвращение результата
        Ok("Authentication successful".to_string())
    }
}
