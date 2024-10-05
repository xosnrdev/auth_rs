use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_email_verified: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "User {{ id: {}, username: {}, email: {}, is_email_verified: {:?}, created_at: {}, updated_at: {} }}",
            self.id, self.username, self.email, self.is_email_verified, self.created_at, self.updated_at
        )
    }
}
