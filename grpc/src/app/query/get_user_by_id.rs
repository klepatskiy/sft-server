use crate::app::error::AppError;
use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::user::user_entity::User;

#[async_trait]
pub trait LoginUserRepository {
    async fn get_user_by_id(&self, uuid: Uuid) -> Result<User, AppError>;
}

pub struct GetFullUrlQuery<R>
where
    R: LoginUserRepository,
{
    repo: R,
}

impl<R> GetFullUrlQuery<R>
where
    R: LoginUserRepository,
{
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn execute(&self, uuid: Uuid) -> Result<User, AppError> {
        self.repo.get_user_by_id(uuid).await
    }
}
