use rusqlite::{Connection, OptionalExtension, params};

use crate::domain::{
    email::Email,
    user::{PasswordHash, User, UserId},
    user_repository::{RepositoryError, UserRepository},
};

pub struct SqliteUserRepository {
    connection: Connection,
}

impl SqliteUserRepository {
    pub fn new(connection: Connection) -> Self {
        SqliteUserRepository { connection }
    }
}

impl UserRepository for SqliteUserRepository {
    fn find_by_email(&self, email: &Email) -> Result<Option<User>, RepositoryError> {
        self.connection
            .query_row(
                "SELECT id, email, password_hash, email_verified FROM users WHERE email = ?1",
                params![email.as_str()],
                |row| {
                    let id: String = row.get(0)?;
                    let email: String = row.get(1)?;
                    let password_hash: String = row.get(2)?;
                    let email_verified: bool = row.get(3)?;
                    Ok((id, email, password_hash, email_verified))
                },
            )
            .optional()
            .map_err(|e| RepositoryError::Failure(e.to_string()))
            .map(|maybe_row| {
                maybe_row.map(|(id, email, password_hash, email_verified)| {
                    User::from_persisted(
                        UserId::new(id),
                        Email::new(&email).expect("stored email should always be valid"),
                        PasswordHash::from_hashed(password_hash),
                        email_verified,
                    )
                })
            })
    }

    fn save(&mut self, user: User) -> Result<(), RepositoryError> {
        self.connection.execute(
            "INSERT INTO users (id, email, password_hash, email_verified) VALUES (?1, ?2, ?3, ?4)",
            params![
                user.id().as_str(),
                user.email().as_str(),
                user.password_hash().as_str(),
                user.is_email_verified(),
            ],
        )
            .map_err(|e| RepositoryError::Failure(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::db;

    #[test]
    fn saves_and_finds_by_email() {
        let connection = db::open(":memory:").unwrap();
        let mut repo = SqliteUserRepository::new(connection);

        let email = Email::new("user@example.com").unwrap();
        let user = User::register(
            UserId::new("u1".into()),
            email.clone(),
            PasswordHash::from_hashed("hash".into()),
        )
        .unwrap();

        repo.save(user).unwrap();

        let found = repo.find_by_email(&email).unwrap();
        assert!(found.is_some());
    }
}
