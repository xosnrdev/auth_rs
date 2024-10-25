use thiserror::Error;
use uuid::Uuid;

use crate::{models::User, repositories::UserRepository};

#[derive(Debug, Error)]
pub enum UserServiceError {
    #[error("User not found")]
    UserNotFound,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

type Result<T> = std::result::Result<T, UserServiceError>;

#[derive(Debug, Clone)]
pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, email: &str, password_hash: &str) -> Result<User> {
        let x_user = self.repository.get_by_email(&email).await?;

        if x_user.is_some() {
            return Err(UserServiceError::UserAlreadyExists);
        }

        let user = User::new(email, password_hash);

        let new_user = self.repository.create(&user).await?;

        Ok(new_user)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = self.repository.get_by_id(id).await?;

        Ok(user)
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = self.repository.get_by_email(email).await?;

        Ok(user)
    }

    pub async fn get_all(&self, limit: i64, offset: i64) -> Result<Vec<User>> {
        let users = self.repository.get_all(limit, offset).await?;

        Ok(users)
    }

    pub async fn update_email(&self, id: Uuid, new_email: &str) -> Result<User> {
        let x_user = self.repository.get_by_id(id).await?;

        match x_user {
            Some(user) => {
                let x_user = self.repository.get_by_email(new_email).await?;

                if x_user.is_some() {
                    return Err(UserServiceError::UserAlreadyExists);
                }

                let updated_user = self.repository.update_email(user.id, new_email).await?;

                Ok(updated_user)
            }
            None => return Err(UserServiceError::UserNotFound),
        }
    }

    pub async fn update_password(&self, id: Uuid, new_password_hash: &str) -> Result<User> {
        let x_user = self.repository.get_by_id(id).await?;

        match x_user {
            Some(user) => {
                let updated_user = self
                    .repository
                    .update_password(user.id, new_password_hash)
                    .await?;

                Ok(updated_user)
            }
            None => return Err(UserServiceError::UserNotFound),
        }
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let x_user = self.repository.get_by_id(id).await?;

        match x_user {
            Some(user) => {
                self.repository.delete(user.id).await?;

                Ok(())
            }
            None => return Err(UserServiceError::UserNotFound),
        }
    }
}
