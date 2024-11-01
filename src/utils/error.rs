use std::convert::From;
use thiserror::Error;
use validator::ValidationErrors;

use crate::{
    dto::{AuthResponse, ErrorDetails},
    middlewares::RateLimitError,
    services::{JWTServiceError, RefreshTokenServiceError, UserServiceError},
};

use super::{HashingError, TokenExtractError};

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

    #[error(transparent)]
    HashingError(#[from] HashingError),

    #[error(transparent)]
    JWTServiceError(#[from] JWTServiceError),

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error("Internal server error")]
    InternalServerError,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Unexpected error occurred")]
    UnexpectedError,
}

impl From<AuthServiceError> for AuthResponse {
    fn from(error: AuthServiceError) -> Self {
        let error_details = match error {
            AuthServiceError::InvalidCredentials => ErrorDetails {
                error: "invalid_credentials",
                error_description: error.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6749#section-4.1.2.1",
            },

            AuthServiceError::TokenExpired => ErrorDetails {
                error: "token_expired",
                error_description: error.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6749#section-4.1.2.1",
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
                error_uri: "https://tools.ietf.org/html/rfc6749#section-4.1.2.1",
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

impl From<TokenExtractError> for AuthResponse {
    fn from(error: TokenExtractError) -> Self {
        let error_details = match error {
            TokenExtractError::MissingAuthorizationHeader => ErrorDetails {
                error: "missing_authorization_header",
                error_description: error.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6750#section-3.1",
            },

            TokenExtractError::InvalidHeaderFormat => ErrorDetails {
                error: "invalid_header_format",
                error_description: error.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6750#section-3.1",
            },

            TokenExtractError::InvalidHeaderContent => ErrorDetails {
                error: "invalid_header_content",
                error_description: error.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6750#section-3.1",
            },
            TokenExtractError::NonUtf8Header(e) => ErrorDetails {
                error: "non_utf8_header",
                error_description: e.to_string(),
                error_uri: "https://tools.ietf.org/html/rfc6750#section-3.1",
            },
        };

        AuthResponse::error(error_details)
    }
}

impl From<RateLimitError> for AuthResponse {
    fn from(error: RateLimitError) -> Self {
        let error_details = ErrorDetails {
            error: "rate_limit_error",
            error_description: error.to_string(),
            error_uri: "https://tools.ietf.org/html/rfc6585#section-4",
        };

        AuthResponse::error(error_details)
    }
}
