use actix_web::{
    http::header::{HeaderValue, ToStrError},
    HttpRequest,
};
use thiserror::Error;

/// Errors that can occur during JWT token extraction
#[derive(Debug, Error)]
pub enum TokenExtractionError {
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
pub mod constants {
    /// The expected prefix for the Authorization header value
    pub const BEARER_PREFIX: &str = "Bearer ";
    /// The name of the Authorization header
    pub const AUTHORIZATION_HEADER: &str = "Authorization";
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
/// * `Err(TokenExtractionError)` - If the token cannot be extracted
///
/// # Example
///
/// ```rust
/// use actix_web::HttpRequest;
///
/// fn handle_request(req: HttpRequest) {
///     match extract_jwt_token(&req) {
///         Ok(token) => println!("Token: {}", token),
///         Err(e) => eprintln!("Error: {}", e),
///     }
/// }
/// ```
pub fn extract_jwt_token(req: &HttpRequest) -> Result<&str, TokenExtractionError> {
    // Get the Authorization header
    let auth_header: &HeaderValue = req
        .headers()
        .get(constants::AUTHORIZATION_HEADER)
        .ok_or(TokenExtractionError::MissingAuthorizationHeader)?;

    // Convert header to string
    let auth_str = auth_header.to_str().map_err(TokenExtractionError::from)?;

    // Validate header format
    if !auth_str.starts_with(constants::BEARER_PREFIX) {
        return Err(TokenExtractionError::InvalidHeaderFormat);
    }

    // Extract token
    let token = &auth_str[constants::BEARER_PREFIX.len()..];

    // Validate token is not empty
    if token.trim().is_empty() {
        return Err(TokenExtractionError::InvalidHeaderContent);
    }

    Ok(token.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn test_missing_authorization_header() {
        let req = TestRequest::default().to_http_request();
        assert!(matches!(
            extract_jwt_token(&req),
            Err(TokenExtractionError::MissingAuthorizationHeader)
        ));
    }

    #[test]
    fn test_invalid_header_format() {
        let req = TestRequest::default()
            .insert_header(("Authorization", "Invalid"))
            .to_http_request();
        assert!(matches!(
            extract_jwt_token(&req),
            Err(TokenExtractionError::InvalidHeaderFormat)
        ));
    }

    #[test]
    fn test_empty_token() {
        let req = TestRequest::default()
            .insert_header(("Authorization", "Bearer "))
            .to_http_request();
        assert!(matches!(
            extract_jwt_token(&req),
            Err(TokenExtractionError::InvalidHeaderContent)
        ));
    }

    #[test]
    fn test_valid_token() {
        let token = "valid.jwt.token";
        let req = TestRequest::default()
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_http_request();
        assert_eq!(extract_jwt_token(&req).unwrap(), token);
    }

    #[test]
    fn test_token_with_whitespace() {
        let token = "valid.jwt.token";
        let req = TestRequest::default()
            .insert_header(("Authorization", format!("Bearer  {}  ", token)))
            .to_http_request();
        assert_eq!(extract_jwt_token(&req).unwrap(), token);
    }
}
