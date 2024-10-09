#[derive(Debug)]
pub struct EmailConfig {
    server_token: String,
    smtp_host: String,
    sender: String,
    verification_token_expiration: i64,
}

impl EmailConfig {
    pub fn new(
        server_token: String,
        smtp_host: String,
        sender: String,
        verification_token_expiration: i64,
    ) -> Self {
        Self {
            server_token,
            smtp_host,
            sender,
            verification_token_expiration,
        }
    }

    pub fn server_token(&self) -> &str {
        &self.server_token
    }

    pub fn smtp_host(&self) -> &str {
        &self.smtp_host
    }

    pub fn sender(&self) -> &str {
        &self.sender
    }

    pub fn verification_token_expiration(&self) -> i64 {
        self.verification_token_expiration
    }
}
