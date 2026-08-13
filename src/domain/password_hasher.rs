use crate::domain::user::PasswordHash;

pub trait PasswordHasher {
    fn hash(&self, plain: &str) -> PasswordHash;
}
