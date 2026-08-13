use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfraError {
    #[error("password hashing failed")]
    Hashing,
    #[error("database error: {0}")]
    Database(String),
}
