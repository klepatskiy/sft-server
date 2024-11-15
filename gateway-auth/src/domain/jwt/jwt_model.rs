use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use strum_macros::{EnumString, AsRefStr};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub token_type: ClaimsTokenType,
    pub issued_at: usize,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, EnumString, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum ClaimsTokenType {
    Access,
    Refresh,
}

impl Claims {
    pub fn new_access(user_id: Uuid, expiration: usize) -> Self {
        Self {
            sub: user_id.to_string(),
            exp: expiration,
            token_type: ClaimsTokenType::Access,
            issued_at: Utc::now().timestamp() as usize,
        }
    }

    pub fn new_refresh(user_id: Uuid, expiration: usize) -> Self {
        Self {
            sub: user_id.to_string(),
            exp: expiration,
            token_type: ClaimsTokenType::Refresh,
            issued_at: Utc::now().timestamp() as usize,
        }
    }
}
