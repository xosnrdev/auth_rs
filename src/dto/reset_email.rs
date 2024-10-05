#[derive(Debug, serde::Deserialize, validator::Validate)]
pub struct ResetEmailDto {
    pub id: uuid::Uuid,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}
