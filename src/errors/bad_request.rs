use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BadRequest {
    message: String,
    timestamp: DateTime<Utc>,
    error_code: u16,
}

impl BadRequest {
    pub fn new(message: String) -> Self {
        Self {
            message,
            timestamp: Utc::now(),
            error_code: 400,
        }
    }
}
