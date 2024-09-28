use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalServerError {
    message: String,
    timestamp: String,
    error_code: u16,
}

impl InternalServerError {
    /// # Summary
    ///
    /// Create a new InternalServerError.
    ///
    /// ## Arguments
    ///
    /// * `message` - The error message.
    ///
    /// ## Example
    ///
    /// ```
    /// use authnorization::errors::InternalServerError;
    /// let internal_server_error = InternalServerError::new("Internal Server Error");
    /// ```
    /// ## Returns
    ///
    /// * `InternalServerError` - The new InternalServerError.
    ///
    pub fn new(message: &str) -> InternalServerError {
        let now: DateTime<Utc> = SystemTime::now().into();
        let now: String = now.to_rfc3339();

        InternalServerError {
            message: String::from(message),
            timestamp: now,
            error_code: 500,
        }
    }
}
