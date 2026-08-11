use argon2::{
    Argon2,
    password_hash::{
        PasswordHash as Argon2Hash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
    },
};

use crate::infrastructure::errors::InfraError;

pub fn hash_password(plain: &str) -> Result<String, InfraError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(plain.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| InfraError::Hashing)
}

pub fn verify_password(plain: &str, hash: &str) -> Result<bool, InfraError> {
    let parsed = Argon2Hash::new(hash).map_err(|_| InfraError::Hashing)?;

    Ok(Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_correct_password() {
        let sut = hash_password("correct horse battery staple").unwrap();

        assert!(verify_password("correct horse battery staple", &sut).unwrap());
    }

    #[test]
    fn rejects_wrong_password() {
        let sut = hash_password("correct horse battery staple").unwrap();
        assert!(!verify_password("wrong password", &sut).unwrap());
    }
}
