use regex::Regex;

type Result<T> = std::result::Result<T, validator::ValidationError>;

pub struct Validation;

impl Validation {
    pub fn email(email: &str) -> Result<()> {
        if !Self::is_valid_email(email) {
            return Err(validator::ValidationError::new("Invalid email address"));
        }
        Ok(())
    }

    pub fn password(password: &str) -> Result<()> {
        if !Self::is_valid_password(password) {
            return Err(validator::ValidationError::new("Password must contain at least one lowercase letter, one uppercase letter, one digit, and one special character"));
        }
        Ok(())
    }

    fn is_valid_email(email: &str) -> bool {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        email_regex.is_match(email)
    }

    fn is_valid_password(password: &str) -> bool {
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

        has_lowercase && has_uppercase && has_digit && has_special
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_passwords() {
        assert!(Validation::password("Password1!").is_ok());
        assert!(Validation::password("P@ssw0rd").is_ok());
        assert!(Validation::password("P@ssw0rd!123").is_ok());
    }

    #[test]
    fn test_invalid_passwords() {
        assert!(Validation::password("").is_err());
        assert!(Validation::password("password").is_err());
        assert!(Validation::password("PASSWORD").is_err());
        assert!(Validation::password("password1").is_err());
        assert!(Validation::password("Password!").is_err());
    }

    #[test]
    fn test_invalid_emails() {
        assert!(Validation::email("user@example").is_err());
        assert!(Validation::email("user.example.com").is_err());
        assert!(Validation::email("user@examplecom").is_err());
        assert!(Validation::email("user@exa$mple.com").is_err());
    }
}
