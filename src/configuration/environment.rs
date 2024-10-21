use super::{ConfigError, Result};

#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum Environment {
    #[default]
    Development,
    Production,
}

impl TryFrom<&str> for Environment {
    type Error = ConfigError;

    fn try_from(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            _ => Err(ConfigError::ParseError("ENVIRONMENT", s.into())),
        }
    }
}
