use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    // Required fields
    status: AuthStatus,
    message: &'static str,

    // Success response fields
    #[serde(skip_serializing_if = "Option::is_none")]
    token_details: Option<TokenDetails>,

    // Error response fields
    #[serde(skip_serializing_if = "Option::is_none")]
    error_details: Option<ErrorDetails>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenDetails {
    token_type: &'static str,
    token: String,
    expires_in: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
    refresh_token: String,
}

impl TokenDetails {
    pub fn new(
        token: impl Into<String>,
        expires_in: i64,
        refresh_token: impl Into<String>,
    ) -> Self {
        Self {
            token_type: "Bearer",
            token: token.into(),
            expires_in,
            scope: None,
            refresh_token: refresh_token.into(),
        }
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetails {
    pub error: &'static str,

    pub error_description: String,

    pub error_uri: &'static str,
}

impl AuthResponse {
    pub fn new(status: AuthStatus, message: &'static str) -> Self {
        Self {
            status,
            message,
            token_details: None,
            error_details: None,
        }
    }

    /// Creates a new successful authentication response
    pub fn success(token_details: TokenDetails) -> Self {
        Self {
            status: AuthStatus::Success,
            message: "Request completed successfully",
            token_details: Some(token_details),
            error_details: None,
        }
    }

    /// Creates a new error authentication response
    pub fn error(error_details: ErrorDetails) -> Self {
        Self {
            status: AuthStatus::Error,
            message: "Request could not be completed",
            token_details: None,
            error_details: Some(error_details),
        }
    }

    /// Returns true if the authentication was successful
    pub fn is_success(&self) -> bool {
        matches!(self.status, AuthStatus::Success)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(email, custom(function = "crate::utils::Validation::email"))]
    #[serde(deserialize_with = "super::deserialize_email")]
    pub email: String,

    #[validate(
        length(min = 8, max = 128),
        custom(function = "crate::utils::Validation::password")
    )]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct AuthenticateDto {
    #[validate(email)]
    #[serde(deserialize_with = "super::deserialize_email")]
    pub email: String,

    pub password: String,
}
