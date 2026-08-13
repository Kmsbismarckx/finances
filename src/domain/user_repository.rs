use thiserror::Error;

use crate::domain::{email::Email, user::User};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RepositoryError {
    #[error("repository operation failed: {0}")]
    Failure(String),
}

pub trait UserRepository {
    fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError>;
    fn save(&mut self, user: User) -> Result<(), RepositoryError>;
}
