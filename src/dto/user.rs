#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct CreateUserDto {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters long"
    ))]
    #[validate(custom(function = "crate::services::ValidationService::validate_username"))]
    pub username: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[validate(custom(function = "crate::services::ValidationService::validate_password"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: String,
}

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct UpdateEmailDto {
    pub id: uuid::Uuid,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct UpdatePasswordDto {
    pub id: uuid::Uuid,

    pub current_password: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[validate(custom(function = "crate::services::ValidationService::validate_password"))]
    pub new_password: String,
}

#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct UpdateUserDto {
    pub id: uuid::Uuid,

    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    #[validate(custom(function = "crate::services::ValidationService::validate_username"))]
    pub username: String,
}
