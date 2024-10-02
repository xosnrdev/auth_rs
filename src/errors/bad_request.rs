use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BadRequest {
    message: String,
    timestamp: DateTime<Utc>,
    error_code: u16,
}

impl BadRequest {
    pub fn new(message: String) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();

        Self {
            message,
            timestamp: now,
            error_code: 400,
        }
    }
}
