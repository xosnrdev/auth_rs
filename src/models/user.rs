use std::fmt;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_id: Option<i64>,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        github_id: Option<i64>,
        email: impl Into<String>,
        password_hash: impl Into<String>,
        username: impl Into<String>,
        avatar_url: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            github_id,
            username: username.into(),
            email: email.into(),
            password_hash: password_hash.into(),
            avatar_url,
            is_admin: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User: {{ id: {}, github_id: {:?}, username: {}, email: {}, avatar_url: {:?}, created_at: {}, updated_at: {} }}",
            self.id,
            self.github_id,
            self.username,
            self.email,
            self.avatar_url,
            self.created_at,
            self.updated_at
        )
    }
}
