use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ReadMessage {
    pub id: Uuid,
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub readed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ReadMessage {
    pub fn new(
        id: Uuid,
        message_id: Uuid,
        user_id: Uuid,
        readed_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            message_id,
            user_id,
            readed_at,
            created_at,
            updated_at,
        }
    }
}
