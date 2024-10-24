use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Debug, thiserror::Error)]
#[error("`HASHING_ERROR` {0}")]
pub struct HashingError(String);

impl From<argon2::password_hash::Error> for HashingError {
    fn from(err: argon2::password_hash::Error) -> Self {
        HashingError(err.to_string())
    }
}

type Result<T> = std::result::Result<T, HashingError>;

pub struct Hashing;

impl Hashing {
    pub fn hash(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn verify(password: &str, hash: &str) -> Result<bool> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_creates_different_hash() {
        let password = "secure_password";
        let hash1 = Hashing::hash(password).unwrap();
        let hash2 = Hashing::hash(password).unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_password_success() {
        let password = "correct_password";
        let hash = Hashing::hash(password).unwrap();
        assert!(Hashing::verify(password, &hash).unwrap());
    }

    #[test]
    fn test_verify_password_failure() {
        let password = "correct_password";
        let wrong_password = "wrong_password";
        let hash = Hashing::hash(password).unwrap();
        assert!(!Hashing::verify(wrong_password, &hash).unwrap());
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let password = "correct_password";
        let invalid_hash = "invalid_hash";
        assert!(Hashing::verify(password, invalid_hash).is_err());
    }
}
