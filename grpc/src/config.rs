use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    // Читаем URL базы данных из переменных окружения
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Создаем пул соединений
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
