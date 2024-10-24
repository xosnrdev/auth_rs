use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RefreshTokenDto {
    pub token: String,
}
