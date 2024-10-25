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
                error_uri: "https://tools.ietf.org/html/rfc6749#section-5.2",
            },

            AuthServiceError::TokenExpired => ErrorDetails {
                error: "token_expired",
                error_description: error.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6749#section-5.2",
            },

            AuthServiceError::UserServiceError(e) => ErrorDetails {
                error: "user_service_error",
                error_description: e.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6749#section-5.2",
            },

            AuthServiceError::RefreshTokenServiceError(e) => ErrorDetails {
                error: "refresh_token_error",
                error_description: e.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6749#section-5.2",
            },

            AuthServiceError::HashingError(e) => ErrorDetails {
                error: "hashing_error",
                error_description: e.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6151",
            },

            AuthServiceError::JWTServiceError(e) => ErrorDetails {
                error: "jwt_error",
                error_description: e.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc7519",
            },

            AuthServiceError::ValidationError(e) => ErrorDetails {
                error: "validation_error",
                error_description: e.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6749#section-5.2",
            },

            AuthServiceError::TokenExtractionError(e) => ErrorDetails {
                error: "token_extraction_error",
                error_description: e.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6750#section-3.1",
            },

            AuthServiceError::InternalServerError => ErrorDetails {
                error: "internal_server_error",
                error_description: error.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6749#section-5.2",
            },

            AuthServiceError::InvalidToken => ErrorDetails {
                error: "invalid_token",
                error_description: error.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6750#section-3.1",
            },

            AuthServiceError::UnexpectedError => ErrorDetails {
                error: "unexpected_error",
                error_description: error.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6749#section-5.2",
            },
        };

        AuthResponse::error(error_details)
    }
}
