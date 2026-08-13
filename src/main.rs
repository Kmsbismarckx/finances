use std::sync::{Arc, Mutex};

use axum::{Router, routing::post};

use finances::{
    infrastructure::{self, user_repository::SqliteUserRepository},
    server::{self, AppState},
};

#[tokio::main]
async fn main() {
    let connection = infrastructure::db::open("finances.db").expect("failed to open database");
    let repo = SqliteUserRepository::new(connection);

    let state = AppState {
        users: Arc::new(Mutex::new(repo)),
    };

    let app = Router::new()
        .route("/auth/register", post(server::auth::register))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind port 3000");

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
