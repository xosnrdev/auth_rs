use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Password hashing error: {0}")]
    HashError(String),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("Environment variable error: {0}")]
    EnvError(#[from] env::VarError),

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Internal server error")]
    InternalServerError,

    #[error("Token expired or invalid")]
    InvalidToken,
}

impl From<argon2::password_hash::Error> for ServiceError {
    fn from(err: argon2::password_hash::Error) -> Self {
        ServiceError::HashError(err.to_string())
    }
}

pub fn validate_password(password: &str) -> Result<(), validator::ValidationError> {
    if !is_valid_password(password) {
        return Err(validator::ValidationError::new("Password must contain at least one lowercase letter, one uppercase letter, one digit, and one special character"));
    }
    Ok(())
}

fn is_valid_password(password: &str) -> bool {
    !password.trim().is_empty()
        && password.chars().any(|c| c.is_ascii_lowercase())
        && password.chars().any(|c| c.is_ascii_uppercase())
        && password.chars().any(|c| c.is_ascii_digit())
        && password.chars().any(|c| !c.is_ascii_alphanumeric())
}

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &PasswordHash) -> bool {
    let argon2 = Argon2::default();
    argon2.verify_password(password.as_bytes(), hash).is_ok()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: i64,
    iat: i64,
    sub: Uuid,
}

pub fn encode_jwt(user_id: Uuid) -> Result<String, ServiceError> {
    let now = Utc::now();
    let claims = Claims {
        exp: (now + Duration::hours(24)).timestamp(),
        iat: now.timestamp(),
        sub: user_id,
    };
    let secret = env::var("JWT_SECRET")?;
    let encoding_key = EncodingKey::from_secret(secret.as_ref());
    encode(&Header::default(), &claims, &encoding_key).map_err(ServiceError::from)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, ServiceError> {
    let secret = env::var("JWT_SECRET")?;
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    decode::<Claims>(token, &decoding_key, &Validation::default()).map_err(ServiceError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_passwords() {
        assert!(validate_password("Password1!").is_ok());
        assert!(validate_password("P@ssw0rd").is_ok());
        assert!(validate_password("P@ssw0rd!123").is_ok());
    }

    #[test]
    fn test_invalid_passwords() {
        assert!(validate_password("").is_err());
        assert!(validate_password("password").is_err());
        assert!(validate_password("PASSWORD").is_err());
        assert!(validate_password("password1").is_err());
        assert!(validate_password("Password!").is_err());
    }

    #[test]
    fn test_hash_password_creates_different_hash() {
        let password = "secure_password";
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_success() {
        let password = "correct_password";
        let hash = hash_password(password).unwrap();
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        assert!(verify_password(password, &parsed_hash));
    }

    #[test]
    fn test_verify_password_failure() {
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let hash = hash_password(password).unwrap();
        let parsed_hash = PasswordHash::new(&hash).unwrap();
        assert!(!verify_password(wrong_password, &parsed_hash));
    }

    #[test]
    fn test_encode_decode_jwt() {
        env::set_var("JWT_SECRET", "test_secret");
        let user_id = Uuid::new_v4();
        let token = encode_jwt(user_id).unwrap();
        let decoded = decode_jwt(&token).unwrap();
        assert_eq!(decoded.claims.sub, user_id);
    }

    #[test]
    fn test_decode_invalid_jwt() {
        env::set_var("JWT_SECRET", "test_secret");
        let result = decode_jwt("invalid_token");
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_expiration() {
        env::set_var("JWT_SECRET", "test_secret");
        let user_id = Uuid::new_v4();
        let token = encode_jwt(user_id).unwrap();
        let decoded = decode_jwt(&token).unwrap();
        assert!(decoded.claims.exp > Utc::now().timestamp());
        assert!(decoded.claims.exp <= (Utc::now() + Duration::hours(24)).timestamp());
    }
}
