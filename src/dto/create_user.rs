use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub email_verified: Option<DateTime<Utc>>,
}
