mod auth;
mod token;
mod user;

pub use auth::*;
pub use token::*;
pub use user::*;
use utils::*;

mod utils {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize_email<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let email = String::deserialize(deserializer)?;
        Ok(email.to_lowercase())
    }
}
