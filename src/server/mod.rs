pub mod auth;

use std::sync::{Arc, Mutex};

use crate::infrastructure::user_repository::SqliteUserRepository;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<SqliteUserRepository>>,
}
