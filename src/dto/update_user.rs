#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct UpdateUserDto {
    pub id: uuid::Uuid,

    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    #[validate(custom(function = "crate::service::validate_username"))]
    pub username: String,

    pub is_email_verified: Option<bool>,
}
