use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};
use log::error;

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: String) -> Result<String, Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                error!("Error hashing password: {}", e);
                e
            })?;

        Ok(password_hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &PasswordHash) -> Result<bool, Error> {
        Argon2::default()
            .verify_password(password.as_bytes(), hash)
            .map_err(|e| {
                error!("Error verifying password: {}", e);
                e
            })?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password = "my_secure_password".to_string();
        let hashed_password = PasswordService::hash_password(password.clone()).unwrap();

        assert_ne!(hashed_password, password);
    }

    #[test]
    fn test_verify_password_success() {
        let password = "my_secure_password".to_string();
        let hashed_password = PasswordService::hash_password(password.clone()).unwrap();
        let parsed_hash = PasswordHash::new(&hashed_password).unwrap();

        let result = PasswordService::verify_password(&password, &parsed_hash).unwrap();
        assert!(result);
    }

    #[test]
    fn test_verify_password_failure() {
        let password = "my_secure_password".to_string();
        let wrong_password = "wrong_password".to_string();
        let hashed_password = PasswordService::hash_password(password.clone()).unwrap();
        let parsed_hash = PasswordHash::new(&hashed_password).unwrap();

        let result = PasswordService::verify_password(&wrong_password, &parsed_hash);
        assert!(result.is_err());
    }
}
