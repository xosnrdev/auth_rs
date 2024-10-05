use argon2::PasswordHash;
use chrono::Utc;
use futures::future::try_join;
use log::{error, info};
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::{RegisterUserDto, ResetEmailDto, ResetPasswordDto, UpdateUserDto},
    models::User,
    repository::UserRepository,
    service::PasswordService,
};

#[derive(Debug, thiserror::Error)]
pub enum UserServiceError {
    #[error("Username '{0}' already exists")]
    UsernameExists(String),
    #[error("Email '{0}' already exists")]
    EmailExists(String),
    #[error("User with ID '{0}' not found")]
    UserNotFound(Uuid),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Password hashing error: {0}")]
    HashError(String),
    #[error("Search query cannot be empty")]
    SearchQuery,
}

impl From<argon2::password_hash::Error> for UserServiceError {
    fn from(err: argon2::password_hash::Error) -> Self {
        UserServiceError::HashError(err.to_string())
    }
}

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }

    pub async fn create(&self, dto: RegisterUserDto) -> Result<User, UserServiceError> {
        dto.validate().map_err(UserServiceError::ValidationError)?;

        let username = dto.username.clone();
        let email = dto.email.clone();

        let (existing_username, existing_email) = try_join(
            self.user_repository.get_by_username(&username),
            self.user_repository.get_by_email(&email),
        )
        .await?;

        if existing_username.is_some() {
            return Err(UserServiceError::UsernameExists(username));
        }

        if existing_email.is_some() {
            return Err(UserServiceError::EmailExists(email));
        }

        let password_hash = PasswordService::hash_password(&dto.password)?;

        let user = User {
            id: Uuid::new_v4(),
            username,
            email,
            password_hash,
            is_email_verified: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let created_user = self.user_repository.create(&user).await?;

        info!("User created successfully: {:?}", created_user);
        Ok(created_user)
    }

    pub async fn update(&self, dto: UpdateUserDto) -> Result<User, UserServiceError> {
        dto.validate().map_err(UserServiceError::ValidationError)?;

        let existing_user = self
            .user_repository
            .get_by_id(&dto.id)
            .await?
            .ok_or_else(|| UserServiceError::UserNotFound(dto.id))?;

        if let Some(other_user) = self.user_repository.get_by_username(&dto.username).await? {
            if other_user.id != dto.id {
                return Err(UserServiceError::UsernameExists(dto.username));
            }
        }

        let updated_user = User {
            id: dto.id,
            username: dto.username,
            email: existing_user.email,
            password_hash: existing_user.password_hash,
            is_email_verified: dto.is_email_verified,
            created_at: existing_user.created_at,
            updated_at: Utc::now(),
        };

        let updated_user = self
            .user_repository
            .update(&updated_user, Utc::now())
            .await?;

        info!("User updated successfully: {:?}", updated_user);
        Ok(updated_user)
    }

    pub async fn change_password(&self, dto: ResetPasswordDto) -> Result<(), UserServiceError> {
        dto.validate().map_err(UserServiceError::ValidationError)?;

        let existing_user = self
            .user_repository
            .get_by_id(&dto.id)
            .await?
            .ok_or_else(|| UserServiceError::UserNotFound(dto.id))?;

        let parsed_hash = PasswordHash::new(&existing_user.password_hash)?;

        if !PasswordService::verify_password(&dto.current_password, &parsed_hash) {
            return Err(UserServiceError::InvalidPassword(
                "Current password is incorrect".to_string(),
            ));
        }

        let new_password_hash = PasswordService::hash_password(&dto.new_password)?;

        self.user_repository
            .update_password(&dto.id, &new_password_hash, Utc::now())
            .await?;

        info!("Password changed successfully for user ID: {}", dto.id);
        Ok(())
    }

    pub async fn update_email(&self, dto: &ResetEmailDto) -> Result<(), UserServiceError> {
        dto.validate().map_err(UserServiceError::ValidationError)?;

        if let Some(existing_email_user) = self.user_repository.get_by_email(&dto.email).await? {
            if existing_email_user.id != dto.id {
                return Err(UserServiceError::EmailExists(dto.email.clone()));
            }
        }

        self.user_repository
            .update_email(&dto.id, &dto.email, Utc::now())
            .await?;

        info!("Email updated successfully for user ID: {}", dto.id);
        Ok(())
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<Option<User>, UserServiceError> {
        self.user_repository.get_by_id(id).await.map_err(|e| {
            error!("Error getting user by id {}: {:?}", id, e);
            UserServiceError::DatabaseError(e)
        })
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, UserServiceError> {
        self.user_repository
            .get_by_username(username)
            .await
            .map_err(|e| {
                error!("Error getting user by username {}: {:?}", username, e);
                UserServiceError::DatabaseError(e)
            })
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, UserServiceError> {
        self.user_repository.get_by_email(email).await.map_err(|e| {
            error!("Error getting user by email {}: {:?}", email, e);
            UserServiceError::DatabaseError(e)
        })
    }

    pub async fn get_all(&self, limit: i64, offset: i64) -> Result<Vec<User>, UserServiceError> {
        self.user_repository
            .get_all(limit, offset)
            .await
            .map_err(|e| {
                error!(
                    "Error getting all users (limit: {}, offset: {}): {:?}",
                    limit, offset, e
                );
                UserServiceError::DatabaseError(e)
            })
    }

    pub async fn delete(&self, id: &Uuid) -> Result<(), UserServiceError> {
        self.user_repository.delete(id).await.map_err(|e| {
            error!("Error deleting user by id {}: {:?}", id, e);
            UserServiceError::DatabaseError(e)
        })
    }

    pub async fn search(
        &self,
        query: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>, UserServiceError> {
        if query.trim().is_empty() {
            return Err(UserServiceError::SearchQuery);
        }

        self.user_repository
            .search(query, limit, offset)
            .await
            .map_err(|e| {
                error!(
                    "Error searching users by query '{}' (limit: {}, offset: {}): {:?}",
                    query, limit, offset, e
                );
                UserServiceError::DatabaseError(e)
            })
    }

    pub async fn count(&self) -> Result<i64, UserServiceError> {
        self.user_repository.count().await.map_err(|e| {
            error!("Error counting users: {:?}", e);

            UserServiceError::DatabaseError(e)
        })
    }

    pub async fn count_search(&self, query: &str) -> Result<i64, UserServiceError> {
        if query.trim().is_empty() {
            return Err(UserServiceError::SearchQuery);
        }

        self.user_repository.count_search(query).await.map_err(|e| {
            error!("Error counting users by query '{}': {:?}", query, e);
            UserServiceError::DatabaseError(e)
        })
    }
}
