use core::str;

use crate::domain::{email::Email, errors::DomainError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn from_hashed(hash: String) -> Self {
        PasswordHash(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(raw: String) -> Self {
        UserId(raw)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct User {
    id: UserId,
    email: Email,
    password_hash: PasswordHash,
    email_verified: bool,
}

impl User {
    pub fn register(
        id: UserId,
        email: Email,
        password_hash: PasswordHash,
    ) -> Result<Self, DomainError> {
        Ok(User {
            id,
            email,
            password_hash,
            email_verified: false,
        })
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn is_email_verified(&self) -> bool {
        self.email_verified
    }

    pub fn mark_email_verified(&mut self) {
        self.email_verified = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_email() -> Email {
        Email::new("user@example.com").unwrap()
    }

    fn sample_hash() -> PasswordHash {
        PasswordHash::from_hashed("hashed-value".to_string())
    }

    #[test]
    fn registers_with_email_unverified() {
        let sut = User::register(UserId::new("u1".into()), sample_email(), sample_hash()).unwrap();

        assert!(!sut.is_email_verified())
    }

    #[test]
    fn mark_email_verified_flips_flag() {
        let mut sut =
            User::register(UserId::new("u1".into()), sample_email(), sample_hash()).unwrap();

        sut.mark_email_verified();
        assert!(sut.is_email_verified())
    }

    #[test]
    fn exposes_the_same_email_it_was_given() {
        let sut = User::register(UserId::new("u1".into()), sample_email(), sample_hash()).unwrap();
        assert_eq!(sut.email().as_str(), "user@example.com");
    }
}
