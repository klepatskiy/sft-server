use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::{FromRow, types::Uuid};
use std::collections::HashMap;

#[derive(Debug, Clone, FromRow)]
pub struct ReadMessage {
    pub id: Uuid,
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub readed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub object_type: String, // Например, "channel" или "user"
    pub object_id: Uuid,
    pub message_text: String,
    pub parent_id: Option<Uuid>,
    pub is_pinned: bool,
    pub config: Value,
    pub edited_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Clone, FromRow)]
pub struct Attachment {
    pub id: Uuid,
    pub message_id: Uuid,
    pub attachments_type: String,
    pub url: String,
    pub metadata: Option<HashMap<String, String>>, 
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ReactionMessage {
    pub id: Uuid,
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub reaction: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}