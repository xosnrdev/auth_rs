#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct ResetPasswordDto {
    pub id: uuid::Uuid,

    pub current_password: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    #[validate(custom(function = "crate::service::validate_password"))]
    pub new_password: String,
}
