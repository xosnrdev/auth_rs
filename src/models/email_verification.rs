use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailVerification {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_used: bool,
}

impl EmailVerification {
    pub fn new(
        id: uuid::Uuid,
        user_id: uuid::Uuid,
        token_hash: String,
        expires_at: DateTime<Utc>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        is_used: bool,
    ) -> Self {
        Self {
            id,
            user_id,
            token_hash,
            expires_at,
            created_at,
            updated_at,
            is_used,
        }
    }
}

impl std::fmt::Display for EmailVerification {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "EmailVerification {{ id: {}, user_id: {}, token_hash: {}, expires_at: {}, created_at: {}, updated_at: {}, is_used: {} }}",
            self.id, self.user_id, self.token_hash, self.expires_at, self.created_at, self.updated_at, self.is_used
        )
    }
}
