#[derive(Clone)]
pub struct EmailService {
    client: reqwest::Client,
    server_token: String,
    smtp_host: String,
    sender: String,
}

impl EmailService {
    pub fn new(server_token: String, smtp_host: String, sender: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            server_token,
            smtp_host,
            sender,
        }
    }

    pub async fn send_verification_email(
        &self,
        to_email: &str,
        verification_link: &str,
    ) -> Result<(), reqwest::Error> {
        let email_content = serde_json::json!({
            "From": self.sender,
            "To": to_email,
            "Subject": "Verify Your Email",
            "TextBody": format!("Please verify your email by clicking on this link: {}", verification_link),
            "HtmlBody": format!("Please verify your email by clicking on this link: <a href=\"{}\">{}</a>", verification_link, verification_link),
            "MessageStream": "outbound"
        });

        let response = self
            .client
            .post(&format!("{}/email", self.smtp_host))
            .header("X-Postmark-Server-Token", &self.server_token)
            .header("Content-Type", "application/json")
            .json(&email_content)
            .send()
            .await?;

        response.error_for_status()?;

        Ok(())
    }
}
