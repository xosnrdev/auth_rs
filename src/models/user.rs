use crate::dto::{CreateUser, RegisterRequest};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt::{Display, Formatter};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        username: String,
        email: Option<String>,
        password: String,
        email_verified: Option<DateTime<Utc>>,
    ) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();

        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password,
            email_verified,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();

        Self {
            id: Uuid::new_v4(),
            username: value.username,
            email: value.email,
            password: value.password,
            email_verified: value.email_verified,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<RegisterRequest> for User {
    fn from(value: RegisterRequest) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();

        Self {
            id: Uuid::new_v4(),
            username: value.username,
            email: value.email,
            password: value.password,
            email_verified: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User {{ id: {}, username: {}, email: {:?}, email_verified: {:?}, created_at: {}, updated_at: {} }}",
               self.id, self.username, self.email, self.email_verified, self.created_at, self.updated_at)
    }
}
