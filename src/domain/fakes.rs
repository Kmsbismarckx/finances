use std::collections::HashMap;

use crate::domain::{
    email::Email,
    password_hasher::PasswordHasher,
    user::{PasswordHash, User},
    user_repository::{RepositoryError, UserRepository},
};

#[derive(Default)]
pub struct InMemoryUserRepository {
    users: HashMap<String, User>,
}

impl UserRepository for InMemoryUserRepository {
    fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError> {
        Ok(self.users.get(email.as_str()).cloned())
    }

    fn save(&mut self, user: User) -> Result<(), RepositoryError> {
        self.users.insert(user.email().as_str().to_string(), user);
        Ok(())
    }
}

pub struct FakePasswordHasher;

impl PasswordHasher for FakePasswordHasher {
    fn hash(&self, plain: &str) -> super::user::PasswordHash {
        PasswordHash::from_hashed(format!("hashed:{plain}"))
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{
        email::Email,
        fakes::InMemoryUserRepository,
        user::{PasswordHash, User, UserId},
        user_repository::UserRepository,
    };

    #[test]
    fn saves_and_finds_by_email() {
        let email = Email::new("user@example.com").unwrap();

        let user = User::register(
            UserId::new("u1".into()),
            email.clone(),
            PasswordHash::from_hashed("hash".into()),
        )
        .unwrap();

        let mut repo = InMemoryUserRepository::default();
        repo.save(user).unwrap();

        assert!(repo.find_by_email(&email).unwrap().is_some());
    }
}
