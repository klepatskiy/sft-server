use chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use sqlx::Type;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    pub nickname: String,
    pub email: String,
    pub avatar: String,
    pub password_hash: String,
    pub gender: Gender,
    pub birthday: Option<DateTime<Utc>>,
    pub status: UserStatus,
    pub roles: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
}

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Online,
    Offline,
    Idle,
    DoNotDisturb,
}