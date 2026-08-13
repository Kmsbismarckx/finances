use crate::{
    domain::{password_hasher::PasswordHasher, user::PasswordHash},
    infrastructure::crypto,
};

pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, plain: &str) -> PasswordHash {
        let hash = crypto::hash_password(plain).expect("argon2 hashing should not fail");
        PasswordHash::from_hashed(hash)
    }
}
