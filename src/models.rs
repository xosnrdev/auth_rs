#[derive(Debug, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct CreateUserDto {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(
        length(min = 8, message = "Password must be at least 8 characters long"),
        custom(function = "crate::services::validate_password")
    )]
    pub password: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, validator::Validate)]
pub struct LoginDto {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    pub password: String,
}
