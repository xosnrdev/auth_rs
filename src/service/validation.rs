use validator::ValidationError;

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
        return Err(ValidationError::new("Password must contain at least one lowercase letter, one uppercase letter, one digit, and one special character"));
    }
    Ok(())
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
        assert!(is_valid_username("username"));
        assert!(is_valid_username("user_name"));
        assert!(is_valid_username("user-name"));
        assert!(is_valid_username("user_name-123"));
    }

    #[test]
    fn test_invalid_usernames() {
        assert!(!is_valid_username(""));
        assert!(!is_valid_username("username!"));
        assert!(!is_valid_username("user name"));
        assert!(!is_valid_username("user@name"));
    }

    #[test]
    fn test_valid_passwords() {
        assert!(is_valid_password("Password1!"));
        assert!(is_valid_password("P@ssw0rd!"));
        assert!(is_valid_password("P@ssw0rd!123"));
    }

    #[test]
    fn test_invalid_passwords() {
        assert!(!is_valid_password(""));
        assert!(!is_valid_password("password"));
        assert!(!is_valid_password("PASSWORD"));
        assert!(!is_valid_password("password1"));
        assert!(!is_valid_password("Password!"));
    }
}
