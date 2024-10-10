use axum::{http::StatusCode, response::IntoResponse};

pub async fn logout() -> impl IntoResponse {
    StatusCode::OK
}
