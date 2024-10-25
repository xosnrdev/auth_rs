use crate::configuration::JWTConfig;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum JWTServiceError {
    /// RFC 6750 Section 3.1: Invalid token format
    #[error("invalid_token: {0}")]
    TokenError(#[from] jsonwebtoken::errors::Error),

    /// RFC 6749 Section 5.2: Invalid token type
    #[error("invalid_request: Invalid token type, expected `{token_type}`, got `{x_token_type}`")]
    InvalidTokenType {
        token_type: String,
        x_token_type: String,
    },
}

pub type Result<T> = std::result::Result<T, JWTServiceError>;

#[derive(Debug, Serialize, Deserialize)]
/// JWT Custom Claims (RFC 7519 Section 4.1)
/// TODO: Add more required claims as recommended by RFC 7519
/// We will go with this for now
pub struct Claims {
    /// RFC 7519 Section 4.1.4: Expiration Time claim
    exp: i64,

    /// RFC 7519 Section 4.1.6: Issued At claim
    iat: i64,
    /// RFC 7519 Section 4.1.2: Subject claim (user identifier)
    sub: Uuid,
    /// User email (custom claim)
    email: String,
    /// RFC 8693 Section 3: Token type
    #[serde(rename = "typ")]
    token_type: String,
}

impl Claims {
    pub fn new(
        user_id: Uuid,
        email: impl Into<String>,
        token_type: TokenType,
        duration: Duration,
    ) -> Self {
        let now = Utc::now();
        Self {
            exp: (now + duration).timestamp(),
            iat: now.timestamp(),
            sub: user_id,
            email: email.into(),
            token_type: token_type.to_string(),
        }
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub const fn get_user_id(&self) -> Uuid {
        self.sub
    }
}

#[derive(Debug)]
pub enum TokenType {
    Access,
    Refresh,
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            Self::Access => "access".to_string(),
            Self::Refresh => "refresh".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JWTService {
    config: JWTConfig,
}

impl JWTService {
    pub const fn new(config: JWTConfig) -> Self {
        Self { config }
    }

    pub fn encode(
        &self,
        user_id: Uuid,
        email: &str,
        token_type: TokenType,
        duration: Duration,
    ) -> Result<String> {
        let claims = Claims::new(user_id, email, token_type, duration);
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.get_secret().as_ref()),
        )
        .map_err(JWTServiceError::from)
    }

    /// RFC 6750 Section 3: Validate Token
    pub fn decode(&self, token: &str, token_type: TokenType) -> Result<TokenData<Claims>> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.get_secret().as_ref()),
            &Validation::default(),
        )?;

        if token_data.claims.token_type != token_type.to_string() {
            return Err(JWTServiceError::InvalidTokenType {
                token_type: token_type.to_string(),
                x_token_type: token_data.claims.token_type,
            });
        }
        Ok(token_data)
    }

    /// Generates an access token following RFC 6749 Section 4.1.4
    pub fn generate_access_token(&self, user_id: Uuid, email: &str) -> Result<String> {
        // RFC 6749 Section 1.4: Access token with limited lifetime
        self.encode(
            user_id,
            email,
            TokenType::Access,
            Duration::minutes(self.config.get_access_token_duration_min()),
        )
    }

    /// Generates a refresh token following RFC 6749 Section 1.5
    pub fn generate_refresh_token(&self, user_id: Uuid, email: &str) -> Result<String> {
        self.encode(
            user_id,
            email,
            TokenType::Refresh,
            Duration::days(self.config.get_refresh_token_duration_day()),
        )
    }

    /// Gets token expiration time following RFC 6749 Section 4.2.2
    pub fn get_expires_in(&self, token: &str) -> Result<i64> {
        let token_data = self.decode(token, TokenType::Access)?;

        // RFC 6749 Section 4.2.2: Expiration time in seconds
        Ok(token_data.claims.exp - token_data.claims.iat)
    }

    /// Validates an access token following RFC 6750 Section 3
    pub fn validate_access_token(&self, token: &str) -> Result<TokenData<Claims>> {
        self.decode(token, TokenType::Access)
    }

    /// Validates a refresh token following RFC 6749 Section 6
    pub fn validate_refresh_token(&self, token: &str) -> Result<TokenData<Claims>> {
        self.decode(token, TokenType::Refresh)
    }
}

/// Unit tests verifying RFC compliance to-be
#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config() -> JWTConfig {
        JWTConfig::new(
            "test_secret",
            7,  // refresh token duration in days
            15, // access token duration in minutes
        )
    }

    fn create_jwt() -> JWTService {
        JWTService::new(create_test_config())
    }

    #[test]
    fn test_rfc6749_token_types() {
        let jwt = create_jwt();
        let user_id = Uuid::new_v4();

        // RFC 6749 Section 1.4: Access Token
        let access_token = jwt
            .generate_access_token(user_id, "user@example.com")
            .unwrap();
        assert!(jwt.validate_access_token(&access_token).is_ok());

        // RFC 6749 Section 1.5: Refresh Token
        let refresh_token = jwt
            .generate_refresh_token(user_id, "user@example.com")
            .unwrap();
        assert!(jwt.validate_refresh_token(&refresh_token).is_ok());
    }

    #[test]
    fn test_rfc7519_claims() {
        let jwt = create_jwt();
        let user_id = Uuid::new_v4();
        let token = jwt
            .generate_access_token(user_id, "user@example.com")
            .unwrap();
        let claims = jwt.validate_access_token(&token).unwrap().claims;

        // RFC 7519 Section 4.1: Registered Claim Names
        assert!(claims.exp > claims.iat);
        assert_eq!(claims.sub, user_id);
    }
}
