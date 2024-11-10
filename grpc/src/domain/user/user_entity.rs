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

// impl FromStr for Gender {
//     type Err = AppError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_lowercase().as_str() {
//             "male" => Ok(Gender::Male),
//             "female" => Ok(Gender::Female),
//             "other" => Ok(Gender::Other),
//             _ => Err(AppError::DatabaseError("Invalid gender value".into())),
//         }
//     }
// }


#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Online,
    Offline,
    Idle,
    DoNotDisturb,
}

// impl FromStr for UserStatus {
//     type Err = AppError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s.to_lowercase().as_str() {
//             "online" => Ok(UserStatus::Online),
//             "offline" => Ok(UserStatus::Offline),
//             "idle" => Ok(UserStatus::Idle),
//             "donotdisturb" => Ok(UserStatus::DoNotDisturb),
//             _ => Err(AppError::DatabaseError("Invalid gender value".into())),
//         }
//     }
// }