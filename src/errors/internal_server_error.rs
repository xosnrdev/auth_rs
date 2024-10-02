use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalServerError {
    message: String,
    timestamp: DateTime<Utc>,
    error_code: u16,
}

impl InternalServerError {
    pub fn new(message: &str) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();

        Self {
            message: String::from(message),
            timestamp: now,
            error_code: 500,
        }
    }
}
