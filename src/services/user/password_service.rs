use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

pub struct PasswordService;

impl PasswordService {
    pub fn new() -> Self {
        Self
    }

    pub fn hash_password(&self, password: &str) -> Result<String, Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &PasswordHash) -> bool {
        let argon2 = Argon2::default();
        argon2.verify_password(password.as_bytes(), hash).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        let password_service = PasswordService::new();
        let password = "my_secure_password".to_string();
        let hashed_password = password_service
            .hash_password(&password)
            .expect("Failed to hash password");

        assert_ne!(hashed_password, password);
    }

    #[test]
    fn test_verify_password_success() {
        let password_service = PasswordService::new();
        let password = "my_secure_password".to_string();
        let password_hash = password_service
            .hash_password(&password)
            .expect("Failed to hash password");
        let parsed_hash = PasswordHash::new(&password_hash).expect("Failed to parse password hash");

        let result = password_service.verify_password(&password, &parsed_hash);
        assert!(result);
    }

    #[test]
    fn test_verify_password_failure() {
        let password_service = PasswordService::new();
        let password = "my_secure_password".to_string();
        let wrong_password = "wrong_password".to_string();
        let password_hash = password_service
            .hash_password(&password)
            .expect("Failed to hash password");
        let parsed_hash = PasswordHash::new(&password_hash).expect("Failed to parse password hash");

        let result = password_service.verify_password(&wrong_password, &parsed_hash);
        assert!(!result);
    }
}
