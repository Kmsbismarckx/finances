use crate::domain::{
    email::Email,
    errors::DomainError,
    password_hasher::PasswordHasher,
    user::{User, UserId},
    user_repository::UserRepository,
};

pub fn register_user(
    repo: &mut impl UserRepository,
    hasher: &impl PasswordHasher,
    id: UserId,
    email: Email,
    plain_password: &str,
) -> Result<User, DomainError> {
    if repo.find_by_email(&email)?.is_some() {
        return Err(DomainError::EmailAlreadyRegistered);
    }

    let password_hash = hasher.hash(plain_password);
    let user = User::register(id, email, password_hash)?;
    repo.save(user.clone())?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::fakes::{FakePasswordHasher, InMemoryUserRepository};

    #[test]
    fn register_new_user() {
        let mut repo = InMemoryUserRepository::default();
        let hasher = FakePasswordHasher;
        let email = Email::new("user@example.com").unwrap();

        let user = register_user(
            &mut repo,
            &hasher,
            UserId::new("u1".into()),
            email.clone(),
            "correct horse battery staple",
        )
        .unwrap();

        assert_eq!(user.email(), &email);
        assert!(repo.find_by_email(&email).unwrap().is_some());
    }

    #[test]
    fn rejects_duplicate_email() {
        let mut repo = InMemoryUserRepository::default();
        let hasher = FakePasswordHasher;
        let email = Email::new("user@example.com").unwrap();

        register_user(
            &mut repo,
            &hasher,
            UserId::new("u1".into()),
            email.clone(),
            "pw1",
        )
        .unwrap();

        let result = register_user(&mut repo, &hasher, UserId::new("u1".into()), email, "pw2");

        assert_eq!(result.unwrap_err(), DomainError::EmailAlreadyRegistered);
    }
}
