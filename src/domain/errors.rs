use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("device name cannot be empty")]
    EmptyDeviceName,
}
