use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BadRequest {
    message: String,
    timestamp: String,
    error_code: u16,
}

impl BadRequest {
    /// # Summary
    ///
    /// Create a new instance of BadRequest.
    ///
    /// ## Arguments
    ///
    /// * `message` - The error message.
    ///
    /// ## Example
    ///
    /// ```
    /// # use authnorization::errors::BadRequest;
    ///
    /// let bad_request = BadRequest::new("Bad Request");
    /// ```
    /// ## Returns
    ///
    /// * `Self` - The new BadRequest.
    ///
    pub fn new(message: &str) -> Self {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        Self {
            message: String::from(message),
            timestamp: now,
            error_code: 400,
        }
    }
}
