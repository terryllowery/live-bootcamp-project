use std::sync::RwLock;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Error, Json};
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{password, AuthAPIError, Email, Password},
};

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let password = match Password::parse(request.password) {
        Ok(password) => password,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let email = match Email::parse(request.email) {
        Ok(email) => email,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    let user_store = state.user_store.read().await;

    if user_store.validate_user(&email, &password).await.is_err() {
        return Err(AuthAPIError::IncorrectCredentails);
    };

    let user = match user_store.get_user(&email).await {
        Ok(user) => user,
        Err(_) => return Err(AuthAPIError::InvalidCredentials),
    };

    Ok((StatusCode::OK.into_response()))
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}
