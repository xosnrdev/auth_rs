use validator::ValidationError;

pub struct ValidationService;

impl ValidationService {
    pub fn validate_username(username: &str) -> Result<(), ValidationError> {
        if !is_valid_username(username) {
            return Err(ValidationError::new(
                "Username can only contain alphanumeric characters, underscores, and hyphens",
            ));
        }
        Ok(())
    }

    pub fn validate_password(password: &str) -> Result<(), ValidationError> {
        if !is_valid_password(password) {
            return Err(ValidationError::new(
                "Password must contain at least one lowercase letter, one uppercase letter, one digit, and one special character",
            ));
        }
        Ok(())
    }
}

fn is_valid_username(username: &str) -> bool {
    !username.trim().is_empty()
        && username
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
}

fn is_valid_password(password: &str) -> bool {
    !password.trim().is_empty()
        && password.chars().any(|c| c.is_ascii_lowercase())
        && password.chars().any(|c| c.is_ascii_uppercase())
        && password.chars().any(|c| c.is_ascii_digit())
        && password.chars().any(|c| !c.is_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_usernames() {
        assert!(ValidationService::validate_username("username").is_ok());
        assert!(ValidationService::validate_username("user_name").is_ok());
        assert!(ValidationService::validate_username("user-name").is_ok());
        assert!(ValidationService::validate_username("user_name-123").is_ok());
    }

    #[test]
    fn test_invalid_usernames() {
        assert!(ValidationService::validate_username("").is_err());
        assert!(ValidationService::validate_username("username!").is_err());
        assert!(ValidationService::validate_username("user name").is_err());
        assert!(ValidationService::validate_username("user@name").is_err());
    }

    #[test]
    fn test_valid_passwords() {
        assert!(ValidationService::validate_password("Password1!").is_ok());
        assert!(ValidationService::validate_password("P@ssw0rd!").is_ok());
        assert!(ValidationService::validate_password("P@ssw0rd!123").is_ok());
    }

    #[test]
    fn test_invalid_passwords() {
        assert!(ValidationService::validate_password("").is_err());
        assert!(ValidationService::validate_password("password").is_err());
        assert!(ValidationService::validate_password("PASSWORD").is_err());
        assert!(ValidationService::validate_password("password1").is_err());
        assert!(ValidationService::validate_password("Password!").is_err());
    }
}
