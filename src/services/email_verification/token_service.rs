use base64::{engine::general_purpose, Engine};
use rand::random;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct TokenService;

impl TokenService {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_token(&self) -> String {
        let random_bytes: [u8; 32] = random();
        general_purpose::URL_SAFE_NO_PAD.encode(random_bytes)
    }

    pub fn hash_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token);
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_token(&self, token: &str, hashed_token: &str) -> bool {
        let calculated_hash = self.hash_token(token);
        calculated_hash == hashed_token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation() {
        let token_service = TokenService::new();
        let token = token_service.generate_token();
        assert!(!token.is_empty(), "Token should not be empty");
    }

    #[test]
    fn test_token_hashing() {
        let token_service = TokenService::new();

        let token = "test_token";
        let hashed_token = token_service.hash_token(token);

        assert!(!hashed_token.is_empty(), "Hashed token should not be empty");
        assert_ne!(
            hashed_token, token,
            "Hashed token should be different from the original token"
        );
    }

    #[test]
    fn test_token_verification() {
        let token_service = TokenService::new();

        let token = "test_token";
        let hashed_token = token_service.hash_token(token);

        assert!(
            token_service.verify_token(token, &hashed_token),
            "Token verification should succeed"
        );
        assert!(
            !token_service.verify_token("wrong_token", &hashed_token),
            "Token verification should fail for a wrong token"
        );
    }
}
