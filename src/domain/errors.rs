use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("device name cannot be empty")]
    EmptyDeviceName,
    #[error("email is already registered")]
    EmailAlreadyRegistered,
    #[error("repository error: {0}")]
    Repository(#[from] crate::domain::user_repository::RepositoryError),
}
