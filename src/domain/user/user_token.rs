use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use crate::domain::user::user_entity::User;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub active: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UserWithToken {
    pub user: User,
    pub user_token: UserToken,
}