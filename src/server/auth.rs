use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::{email::Email, errors::DomainError, register_user::register_user, user::UserId},
    infrastructure::password_hasher::Argon2PasswordHasher,
    server::AppState,
};

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    id: String,
    email: String,
}

pub struct ApiError(StatusCode, String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorBody {
            error: String,
        }
        (self.0, Json(ErrorBody { error: self.1 })).into_response()
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        let status = match &err {
            DomainError::InvalidEmail | DomainError::EmptyDeviceName => StatusCode::BAD_REQUEST,
            DomainError::EmailAlreadyRegistered => StatusCode::CONFLICT,
            DomainError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError(status, err.to_string())
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<RegisterResponse>), ApiError> {
    let email = Email::new(&payload.email)?;
    let hasher = Argon2PasswordHasher;
    let id = UserId::new(Uuid::new_v4().to_string());

    let mut repo = state.users.lock().expect("users mutex posisoned");
    let user = register_user(&mut *repo, &hasher, id, email, &payload.password)?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            id: user.id().as_str().to_string(),
            email: user.email().as_str().to_string(),
        }),
    ))
}
