use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalServerError {
    message: String,
    timestamp: DateTime<Utc>,
    error_code: u16,
}

impl InternalServerError {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
            timestamp: Utc::now(),
            error_code: 500,
        }
    }
}
