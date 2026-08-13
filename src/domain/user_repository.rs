use crate::domain::{email::Email, user::User};

pub trait UserRepository {
    fn find_by_email(&self, email: &Email) -> Option<User>;
    fn save(&mut self, user: User);
}
