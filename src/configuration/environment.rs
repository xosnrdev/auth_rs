#[derive(Debug, Clone, PartialEq, Copy, Default)]
pub enum Environment {
    #[default]
    Development,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Development => "development",
            Self::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `development` or `production`",
                other
            )),
        }
    }
}
