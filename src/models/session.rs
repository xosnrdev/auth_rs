use std::fmt;

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: String,
    pub is_revoked: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: Uuid, refresh_token: impl Into<String>, duration: Duration) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            refresh_token: refresh_token.into(),
            is_revoked: false,
            expires_at: Utc::now() + duration,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

impl fmt::Display for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Session: {{ id: {}, user_id: {}, refresh_token: {}, is_revoked: {}, expires_at: {}, created_at: {}, updated_at: {} }}",
            self.id,
            self.user_id,
            self.refresh_token,
            self.is_revoked,
            self.expires_at,
            self.created_at,
            self.updated_at
        )
    }
}
