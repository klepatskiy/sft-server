use std::sync::Arc;
use sqlx::PgPool;
use crate::app::error::AppError;
use crate::domain::user::user_token::{UserToken, UserWithToken};
use sqlx::Error as SqlxError;
use crate::domain::user::user_entity::{Gender, User, UserStatus};

#[derive(Clone)]
pub struct PostgresUserTokenRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserTokenRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create_user_token(&self, user_token: UserToken) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO users_token (id, user_id, token, active, expires_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            user_token.id,
            user_token.user_id,
            user_token.token,
            user_token.active,
            user_token.expires_at,
            user_token.created_at,
            user_token.updated_at
        )
            .execute(&*self.pool)
            .await
            .map_err(|err| AppError::DatabaseError(err.into()))?;

        Ok(())
    }

    pub async fn get_user_token(&self, user_token: &str) -> Result<UserWithToken, AppError> {
        let row = sqlx::query!(
            r#"
            SELECT
                u.id as user_id,
                u.first_name,
                u.last_name,
                u.nickname,
                u.password_hash,
                u.email,
                u.avatar,
                u.gender as "gender: Gender",
                u.birthday,
                u.status as "status: UserStatus",
                u.roles,
                u.created_at as user_created_at,
                u.updated_at as user_updated_at,

                ut.id as user_token_id,
                ut.user_id as user_token_user_id,
                ut.token,
                ut.active,
                ut.expires_at,
                ut.created_at as user_token_created_at,
                ut.updated_at as user_token_updated_at
            FROM users_token ut
            JOIN users u ON ut.user_id = u.id
            WHERE ut.token = $1
            "#,
            user_token
        )
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => AppError::NotFound,
                _ => AppError::DatabaseError(err.into()),
            })?;

        let user = User {
            id: row.user_id,
            first_name: row.first_name,
            last_name: row.last_name,
            nickname: row.nickname,
            email: row.email,
            avatar: row.avatar,
            password_hash: row.password_hash,
            gender: row.gender,
            birthday: row.birthday,
            status: row.status,
            roles: row.roles,
            created_at: row.user_created_at,
            updated_at: row.user_updated_at,
        };

        let user_token = UserToken {
            id: row.user_token_id,
            user_id: row.user_token_user_id,
            token: row.token,
            active: row.active,
            expires_at: row.expires_at,
            created_at: row.user_token_created_at,
            updated_at: row.user_token_updated_at,
        };

        Ok(UserWithToken { user, user_token })
    }
}
