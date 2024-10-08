use mongodb::bson::oid::ObjectId;
use thiserror::Error;

use super::dto::UserSignUpRequest;
use super::models::User;
use super::repository::UserRepository;

#[derive(Error, Debug)]
pub enum UserServiceError {
    #[error("User with this email already exists")]
    UserAlreadyExists,
    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
}

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub fn new(repository: UserRepository) -> Self {
        UserService { repository }
    }

    pub async fn create_user(&self, data: UserSignUpRequest) -> Result<ObjectId, UserServiceError> {
        if (self.repository.find_user_by_email(&data.email).await?).is_some() {
            return Err(UserServiceError::UserAlreadyExists);
        }

        let new_user = User {
            id: None,
            name: data.name,
            email: data.email,
            password: data.password, // TODO: hash password
            phone: data.phone,
        };
        self.repository
            .create_user(new_user)
            .await
            .map_err(UserServiceError::from)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, UserServiceError> {
        self.repository
            .find_user_by_email(email)
            .await
            .map_err(UserServiceError::from)
    }
}
