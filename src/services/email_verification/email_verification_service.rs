use crate::models::EmailVerification;
use crate::repositories::EmailVerificationRepository;
use chrono::{Duration, Utc};

use super::TokenService;

#[derive(Debug, thiserror::Error)]
pub enum EmailVerificationError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Email sending error: {0}")]
    EmailError(#[from] reqwest::Error),
    #[error("Verification not found")]
    VerificationNotFound,
    #[error("Verification expired")]
    VerificationExpired,
    #[error("Verification already used")]
    VerificationAlreadyUsed,
}

pub struct EmailVerificationService {
    email_verification_repository: EmailVerificationRepository,
    token_service: TokenService,
    verification_token_expiration: i64,
}

impl EmailVerificationService {
    pub fn new(
        email_verification_repository: EmailVerificationRepository,
        token_service: TokenService,
        verification_token_expiration: i64,
    ) -> Self {
        Self {
            email_verification_repository,
            token_service,
            verification_token_expiration,
        }
    }

    pub async fn create_verification(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<EmailVerification, EmailVerificationError> {
        log::info!("Creating email verification for user: {}", user_id);

        let user_exists = self
            .email_verification_repository
            .get_by_user_id(user_id)
            .await
            .map_err(EmailVerificationError::DatabaseError)?
            .is_some();

        if !user_exists {
            return Err(EmailVerificationError::VerificationNotFound);
        }

        let token = self.token_service.generate_token();
        let token_hash = self.token_service.hash_token(&token);

        let verification = EmailVerification::new(
            uuid::Uuid::new_v4(),
            user_id,
            token_hash,
            Utc::now() + Duration::hours(self.verification_token_expiration),
            Utc::now(),
            Utc::now(),
            false,
        );

        self.email_verification_repository
            .create(&verification)
            .await
            .map_err(EmailVerificationError::DatabaseError)?;

        log::info!("Email verification created for user: {}", user_id);
        Ok(verification)
    }

    pub async fn verify_email(
        &self,
        token: &str,
    ) -> Result<EmailVerification, EmailVerificationError> {
        log::info!("Verifying email with token: {}", token);

        let token_hash = self.token_service.hash_token(token);

        let verification = self
            .email_verification_repository
            .get_by_token_hash(token_hash)
            .await?
            .ok_or(EmailVerificationError::VerificationNotFound)?;

        if verification.is_used {
            return Err(EmailVerificationError::VerificationAlreadyUsed);
        }

        if verification.expires_at < Utc::now() {
            return Err(EmailVerificationError::VerificationExpired);
        }

        self.email_verification_repository
            .update_is_used(verification.id, true, Utc::now())
            .await?;

        self.email_verification_repository
            .update_is_email_verified(verification.user_id, true, Utc::now())
            .await?;

        log::info!("Email verified for user: {:?}", verification.user_id);

        Ok(verification)
    }

    pub async fn resend_verification(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<EmailVerification, EmailVerificationError> {
        log::info!("Resending verification for user: {}", user_id);

        if let Some(existing_verification) = self
            .email_verification_repository
            .get_by_user_id(user_id)
            .await?
        {
            if !existing_verification.is_used && existing_verification.expires_at > Utc::now() {
                return Err(EmailVerificationError::VerificationAlreadyUsed);
            }

            self.email_verification_repository
                .delete(existing_verification.id)
                .await?;
        }

        let verification = self.create_verification(user_id).await?;

        log::info!("Verification resent for user: {}", user_id);

        Ok(verification)
    }

    pub async fn cleanup_expired_verifications(&self) -> Result<(), EmailVerificationError> {
        log::info!("Cleaning up expired verifications");
        self.email_verification_repository
            .delete_expired_verifications()
            .await?;
        Ok(())
    }
}
