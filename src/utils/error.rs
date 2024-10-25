use std::convert::From;

use thiserror::Error;
use validator::ValidationErrors;

use crate::{
    dto::{AuthResponse, ErrorDetails},
    services::{JWTServiceError, RefreshTokenServiceError, UserServiceError},
};

use super::{HashingError, TokenExtractionError};

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Invalid credentials provided")]
    InvalidCredentials,

    #[error("The provided token has expired")]
    TokenExpired,

    #[error(transparent)]
    UserServiceError(#[from] UserServiceError),

    #[error(transparent)]
    RefreshTokenServiceError(#[from] RefreshTokenServiceError),

    #[error("transparent")]
    HashingError(#[from] HashingError),

    #[error("transparent")]
    JWTServiceError(#[from] JWTServiceError),

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    TokenExtractionError(#[from] TokenExtractionError),

    #[error("An internal server error occurred")]
    InternalServerError,

    #[error("The token provided is invalid")]
    InvalidToken,

    #[error("An unexpected error occurred")]
    UnexpectedError,
}

impl From<AuthServiceError> for AuthResponse {
    fn from(error: AuthServiceError) -> Self {
        let error_details = match error {
            AuthServiceError::InvalidCredentials => ErrorDetails {
                error: "invalid_credentials",
                error_description: error.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6749#section-5.2".to_string()),
            },

            AuthServiceError::TokenExpired => ErrorDetails {
                error: "token_expired",
                error_description: error.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6749#section-5.2".to_string()),
            },

            AuthServiceError::UserServiceError(e) => ErrorDetails {
                error: "user_service_error",
                error_description: e.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6749#section-5.2".to_string()),
            },

            AuthServiceError::RefreshTokenServiceError(e) => ErrorDetails {
                error: "refresh_token_error",
                error_description: e.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6749#section-5.2".to_string()),
            },

            AuthServiceError::HashingError(e) => ErrorDetails {
                error: "hashing_error",
                error_description: e.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6151".to_string()),
            },

            AuthServiceError::JWTServiceError(e) => ErrorDetails {
                error: "jwt_error",
                error_description: e.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc7519".to_string()),
            },

            AuthServiceError::ValidationError(e) => ErrorDetails {
                error: "validation_error",
                error_description: e.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6749#section-5.2".to_string()),
            },

            AuthServiceError::TokenExtractionError(e) => ErrorDetails {
                error: "token_extraction_error",
                error_description: e.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6750#section-3.1".to_string()),
            },

            AuthServiceError::InternalServerError => ErrorDetails {
                error: "internal_server_error",
                error_description: error.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6749#section-5.2".to_string()),
            },

            AuthServiceError::InvalidToken => ErrorDetails {
                error: "invalid_token",
                error_description: error.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6750#section-3.1".to_string()),
            },

            AuthServiceError::UnexpectedError => ErrorDetails {
                error: "unexpected_error",
                error_description: error.to_string(),
                error_uri: Some("https://tools.ietf.org/html/rfc6749#section-5.2".to_string()),
            },
        };

        AuthResponse::error(error_details)
    }
}
