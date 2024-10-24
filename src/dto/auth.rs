use serde::Serialize;

#[derive(Debug, Serialize)]
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
pub struct TokenDetails {
    token_type: &'static str,
    token: String,
    expires_in: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
}

impl TokenDetails {
    pub fn new(
        token: impl Into<String>,
        expires_in: i64,
        refresh_token: impl Into<Option<String>>,
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
pub struct ErrorDetails {
    pub error: &'static str,

    pub error_description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_uri: Option<String>,
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
    pub fn success(token_details: TokenDetails, message: &'static str) -> Self {
        Self {
            status: AuthStatus::Success,
            message,
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
