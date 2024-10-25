use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmailDto {
    #[validate(email, custom(function = "crate::utils::Validation::email"))]
    #[serde(rename = "lowercase")]
    pub new_email: String,
    pub access_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePasswordDto {
    #[validate(
        length(min = 8, max = 128),
        custom(function = "crate::utils::Validation::password")
    )]
    pub new_password: String,
    pub access_token: String,
}
