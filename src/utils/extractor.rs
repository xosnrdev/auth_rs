use std::{
    future::{ready, Future},
    pin::Pin,
};

use actix_web::{dev::Payload, http::header::ToStrError, Error, FromRequest, HttpRequest};
use serde::Deserialize;

/// Errors that can occur during JWT token extraction
#[derive(Debug, thiserror::Error)]
pub enum TokenExtractError {
    #[error("Authorization header not found")]
    MissingAuthorizationHeader,

    #[error("Invalid Authorization header format: expected 'Bearer <token>'")]
    InvalidHeaderFormat,

    #[error("Invalid Authorization header content: token is empty")]
    InvalidHeaderContent,

    #[error("Header contains non-UTF8 characters")]
    NonUtf8Header(#[from] ToStrError),
}

/// Constants for JWT token extraction
mod constants {
    /// The expected prefix for the Authorization header value
    pub const BEARER_PREFIX: &str = "Bearer ";
    /// The name of the Authorization header
    pub const AUTHORIZATION_HEADER: &'static str = "Authorization";
}

/// Type to hold the extracted JWT token
#[derive(Debug, Deserialize)]
pub struct TokenExtract(pub String);

impl TokenExtract {
    /// Get the token string
    pub fn get_token(&self) -> &str {
        &self.0
    }

    /// Extracts a JWT token from the Authorization header of an HTTP request.
    ///
    /// # Arguments
    ///
    /// * `req` - The HTTP request to extract the token from
    ///
    /// # Returns
    ///
    /// * `Ok(&str)` - The extracted JWT token
    /// * `Err(TokenExtractError)` - If the token cannot be extracted
    pub fn extract(req: &HttpRequest) -> Result<String, TokenExtractError> {
        // Get the Authorization header
        let auth_header = req
            .headers()
            .get(constants::AUTHORIZATION_HEADER)
            .ok_or(TokenExtractError::MissingAuthorizationHeader)?;

        // Convert header to string
        let auth_str = auth_header.to_str()?;

        // Validate header format
        if !auth_str.starts_with("Bearer ") {
            return Err(TokenExtractError::InvalidHeaderFormat);
        }

        // Extract token
        let token = &auth_str[constants::BEARER_PREFIX.len()..];

        // Validate token is not empty
        if token.trim().is_empty() {
            return Err(TokenExtractError::InvalidHeaderContent);
        }

        Ok(token.trim().to_string())
    }
}

impl FromRequest for TokenExtract {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Wrap the result in a future
        let fut = match TokenExtract::extract(req) {
            Ok(token) => ready(Ok(TokenExtract(token))),
            Err(e) => ready(Err(actix_web::error::ErrorUnauthorized(e))),
        };

        Box::pin(fut)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn test_missing_authorization_header() {
        let req = TestRequest::default().to_http_request();
        assert!(matches!(
            TokenExtract::extract(&req),
            Err(TokenExtractError::MissingAuthorizationHeader)
        ));
    }

    #[test]
    fn test_invalid_header_format() {
        let req = TestRequest::default()
            .insert_header(("Authorization", "Invalid"))
            .to_http_request();
        assert!(matches!(
            TokenExtract::extract(&req),
            Err(TokenExtractError::InvalidHeaderFormat)
        ));
    }

    #[test]
    fn test_empty_token() {
        let req = TestRequest::default()
            .insert_header(("Authorization", "Bearer "))
            .to_http_request();
        assert!(matches!(
            TokenExtract::extract(&req),
            Err(TokenExtractError::InvalidHeaderContent)
        ));
    }

    #[test]
    fn test_valid_token() {
        let token = "valid.jwt.token";
        let req = TestRequest::default()
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_http_request();
        assert_eq!(TokenExtract::extract(&req).unwrap(), token);
    }

    #[test]
    fn test_token_with_whitespace() {
        let token = "valid.jwt.token";
        let req = TestRequest::default()
            .insert_header(("Authorization", format!("Bearer  {}  ", token)))
            .to_http_request();
        assert_eq!(TokenExtract::extract(&req).unwrap(), token);
    }
}
