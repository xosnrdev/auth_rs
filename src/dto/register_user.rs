#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct RegisterUserDto {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters long"
    ))]
    #[validate(custom(function = "crate::service::validate_username"))]
    pub username: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[validate(custom(function = "crate::service::validate_password"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: String,
}
