#[derive(Debug, serde::Deserialize)]
pub struct CreateEmailVerificationDto {
    pub user_id: uuid::Uuid,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateEmailVerificationDto {
    pub is_used: bool,
}
