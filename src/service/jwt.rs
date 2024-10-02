use crate::configuration::JwtConfig;
use jsonwebtoken::{encode, errors::Error as JwtError, DecodingKey, EncodingKey, Header};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    iat: i64,
    sub: String,
}

impl Claims {
    pub fn new(sub: String, exp: i64, iat: i64) -> Self {
        Self { sub, exp, iat }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("JWT error: {0}")]
    JwtError(#[from] JwtError),
    #[error("Token expired")]
    TokenExpired,
}

pub struct JwtService {
    jwt_config: JwtConfig,
}

impl JwtService {
    pub fn new(jwt_config: JwtConfig) -> Self {
        Self { jwt_config }
    }

    pub fn generate_jwt_token(&self, subject: &str) -> Result<String, Error> {
        let now = chrono::Utc::now();
        let exp = now + chrono::Duration::seconds(self.jwt_config.jwt_expiration());
        let iat = now;

        let claims = Claims::new(subject.to_string(), exp.timestamp(), iat.timestamp());

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.jwt_secret().as_bytes()),
        )
        .map_err(|e| {
            error!("Error generating JWT token: {}", e);
            Error::JwtError(e)
        })?;

        Ok(token)
    }

    pub fn verify_jwt_token(&self, token: &str) -> Result<String, Error> {
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_config.jwt_secret().as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|e| {
            error!("Error verifying JWT token: {}", e);
            Error::JwtError(e)
        })?;

        if token_data.claims.exp < chrono::Utc::now().timestamp() {
            return Err(Error::TokenExpired);
        }

        Ok(token_data.claims.sub)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::JwtConfig;

    #[test]
    fn test_generate_jwt_token() {
        let jwt_config = JwtConfig::new("my_secret".to_string(), 5);
        let jwt_service = JwtService::new(jwt_config);
        let subject = "test_user";

        let token = jwt_service.generate_jwt_token(subject).unwrap();

        assert!(!token.is_empty());
    }

    #[test]
    fn test_verify_jwt_token_success() {
        let jwt_config = JwtConfig::new("my_secret".to_string(), 5);
        let jwt_service = JwtService::new(jwt_config);
        let subject = "test_user";

        let token = jwt_service.generate_jwt_token(subject).unwrap();
        let verified_subject = jwt_service.verify_jwt_token(&token).unwrap();

        assert_eq!(verified_subject, subject);
    }

    #[test]
    fn test_verify_jwt_token_failure() {
        let jwt_config = JwtConfig::new("my_secret".to_string(), 5);
        let jwt_service = JwtService::new(jwt_config);
        let subject = "test_user";

        let token = jwt_service.generate_jwt_token(subject).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(6));

        let result = jwt_service.verify_jwt_token(&token);
        assert!(result.is_err());
    }
}
