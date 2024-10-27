use std::fmt;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    #[serde(skip_serializing)]
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_revoked: bool,
}

impl RefreshToken {
    pub fn new(user_id: Uuid, token: impl Into<String>, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            token: token.into(),
            expires_at,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            is_revoked: false,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub const fn is_revoked(&self) -> bool {
        self.is_revoked
    }

    pub const fn get_user_id(&self) -> Uuid {
        self.user_id
    }
}

impl fmt::Display for RefreshToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RefreshToken {{ id: {}, user_id: {}, expires_at: {}, created_at: {}, is_revoked: {} }}",
            self.id, self.user_id, self.expires_at, self.created_at, self.is_revoked
        )
    }
}
