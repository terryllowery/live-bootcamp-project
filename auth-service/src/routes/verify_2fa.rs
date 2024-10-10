use axum::{http::StatusCode, response::IntoResponse};

pub async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK
}
