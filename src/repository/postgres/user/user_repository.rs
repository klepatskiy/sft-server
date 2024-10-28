use crate::app::error::AppError;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use crate::domain::user::user_entity::User;
use sqlx::Error as SqlxError;

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl crate::app::command::login_user_command::LoginUserRepository for PostgresUserRepository {
    async fn get_user(&self, email: String) -> Result<User, AppError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT
                id
                , first_name
                , last_name
                , nickname
                , password_hash
                , email
                , avatar
                , gender as "gender: _"
                , birthday
                , status as "status: _"
                , roles
                , created_at
                , updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
            .fetch_one(&*self.pool)
            .await
            .map_err(|err| match err {
                SqlxError::RowNotFound => AppError::NotFound,
                _ => AppError::DatabaseError(err.into()),
            })?;

        Ok(user)
    }
}

// #[async_trait]
// impl crate::app::query::get_full_url::LoginUserRepository for PostgresUrlRepository {
//     async fn get_full_url(&self, short_url: String) -> Result<Url, AppError> {
//         let query = "
//             SELECT id, url_full, url_short, user_id, created_at
//             FROM urls
//             WHERE url_short = $1
//             AND user_id IS NULL
//         ";
//
//         let url = sqlx::query_as::<_, Url>(query)
//             .bind(short_url)
//             .fetch_one(&*self.pool)
//             .await
//             .map_err(AppError::DatabaseError)?;
//
//         Ok(url)
//     }
// }
