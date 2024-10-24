use crate::configuration::JWTConfig;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
#[error("`JWT_ERROR` {0}")]
pub struct JWTError(#[from] jsonwebtoken::errors::Error);

pub type Result<T> = std::result::Result<T, JWTError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    iat: i64,
    sub: Uuid,
    email: String,
}

impl Claims {
    pub fn new(user_id: Uuid, email: impl Into<String>, duration: Duration) -> Self {
        let now = Utc::now();
        Self {
            exp: (now + duration).timestamp(),
            iat: now.timestamp(),
            sub: user_id,
            email: email.into(),
        }
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}

pub struct JWT {
    config: JWTConfig,
}

impl JWT {
    pub const fn new(config: JWTConfig) -> Self {
        Self { config }
    }

    pub fn encode(&self, user_id: Uuid, email: &str, duration: Duration) -> Result<String> {
        let claims = Claims::new(user_id, email, duration);
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.get_secret().as_ref()),
        )
        .map_err(JWTError)
    }

    pub fn decode(&self, token: &str) -> Result<TokenData<Claims>> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.get_secret().as_ref()),
            &Validation::default(),
        )?;
        Ok(token_data)
    }

    pub fn generate_access_token(&self, user_id: Uuid, email: &str) -> Result<String> {
        self.encode(
            user_id,
            email,
            Duration::minutes(self.config.get_access_token_duration_min()),
        )
    }

    pub fn generate_refresh_token(&self, user_id: Uuid, email: &str) -> Result<String> {
        self.encode(
            user_id,
            email,
            Duration::days(self.config.get_refresh_token_duration_day()),
        )
    }

    pub fn get_expires_in(&self, token: &str) -> Result<i64> {
        let token_data = self.decode(token)?;
        Ok(token_data.claims.exp - Utc::now().timestamp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_jwt() -> JWT {
        let config = JWTConfig::new("test_secret", 7, 15);
        JWT::new(config)
    }

    #[test]
    fn test_encode_decode_jwt() {
        let jwt = create_jwt();
        let user_id = Uuid::new_v4();
        let token = jwt
            .encode(user_id, "user@example.com", Duration::minutes(24))
            .unwrap();
        let decoded = jwt.decode(&token).unwrap();
        assert_eq!(decoded.claims.sub, user_id);
    }

    #[test]
    fn test_decode_invalid_jwt() {
        let jwt = create_jwt();
        let result = jwt.decode("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_expiration() {
        let jwt = create_jwt();
        let user_id = Uuid::new_v4();
        let token = jwt
            .generate_refresh_token(user_id, "user@example.com")
            .unwrap();
        let decoded = jwt.decode(&token).unwrap();
        assert!(decoded.claims.exp > Utc::now().timestamp());
        assert!(decoded.claims.exp <= (Utc::now() + Duration::days(7)).timestamp());
    }
}
