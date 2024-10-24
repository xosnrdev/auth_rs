use std::convert::From;

use thiserror::Error;

use crate::{
    dto::{AuthResponse, ErrorDetails},
    services::{RefreshTokenServiceError, UserServiceError},
};

use super::{HashingError, JWTError};

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
    JWTError(#[from] JWTError),

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
                error_uri: None,
            },

            AuthServiceError::TokenExpired => ErrorDetails {
                error: "token_expired",
                error_description: error.to_string(),
                error_uri: None,
            },

            AuthServiceError::UserServiceError(e) => ErrorDetails {
                error: "user_service_error",
                error_description: e.to_string(),
                error_uri: None,
            },

            AuthServiceError::RefreshTokenServiceError(e) => ErrorDetails {
                error: "refresh_token_error",
                error_description: e.to_string(),
                error_uri: None,
            },

            AuthServiceError::HashingError(e) => ErrorDetails {
                error: "hashing_error",
                error_description: e.to_string(),
                error_uri: None,
            },

            AuthServiceError::JWTError(e) => ErrorDetails {
                error: "jwt_error",
                error_description: e.to_string(),
                error_uri: None,
            },

            AuthServiceError::InternalServerError => ErrorDetails {
                error: "internal_server_error",
                error_description: error.to_string(),
                error_uri: None,
            },

            AuthServiceError::InvalidToken => ErrorDetails {
                error: "invalid_token",
                error_description: error.to_string(),
                error_uri: None,
            },

            AuthServiceError::UnexpectedError => ErrorDetails {
                error: "unexpected_error",
                error_description: error.to_string(),
                error_uri: None,
            },
        };

        AuthResponse::error(error_details)
    }
}

pub trait IntoAuthResponse<T> {
    fn into_auth_response(self) -> AuthResponse;
}

impl<T> IntoAuthResponse<T> for Result<T, AuthServiceError> {
    fn into_auth_response(self) -> AuthResponse {
        match self {
            Ok(_) => panic!("into_auth_response called on Ok variant"),
            Err(e) => e.into(),
        }
    }
}
