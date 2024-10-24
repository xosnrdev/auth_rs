use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    // Required fields
    pub status: AuthStatus,
    pub message: &'static str,

    // Success response fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_details: Option<TokenDetails>,

    // Error response fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize)]
pub struct TokenDetails {
    pub token_type: String,
    pub token: String,
    pub expires_in: Duration,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub error: &'static str,

    pub error_description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,
}

impl AuthResponse {
    /// Creates a new successful authentication response
    pub fn success(token_details: TokenDetails) -> Self {
        Self {
            status: AuthStatus::Success,
            message: "Authentication successful",
            token_details: Some(token_details),
            error_details: None,
        }
    }

    /// Creates a new error authentication response
    pub fn error(error_details: ErrorDetails) -> Self {
        Self {
            status: AuthStatus::Error,
            message: "Authentication failed",
            token_details: None,
            error_details: Some(error_details),
        }
    }

    /// Returns true if the authentication was successful
    pub fn is_success(&self) -> bool {
        matches!(self.status, AuthStatus::Success)
    }
}
