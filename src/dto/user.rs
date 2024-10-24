use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "lowercase")]
pub struct RegisterDto {
    #[validate(email, custom(function = "crate::utils::Validation::email"))]
    pub email: String,

    #[validate(
        length(min = 8, max = 128),
        custom(function = "crate::utils::Validation::password")
    )]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "lowercase")]
pub struct LoginDto {
    #[validate(email)]
    pub email: String,

    pub password: String,
}
