use std::error::Error;
use tower_http::services::ServeDir;
use axum::{
    http::{response, StatusCode}, response::IntoResponse, routing::{get, post, Router}, serve::Serve
};

pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/logout", post(logout))
            .route("/verify-2fa", post(verify_2fa))
            .route("/verify-token", post(verify_token));


        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("Listening on {}", self.address);
        self.server.await
    }
}
// TODO: Implement the signup, login, logout, verify-2fa, and verify-token handlers
async fn signup() -> impl IntoResponse {
    StatusCode::OK
}

async fn login() -> impl IntoResponse {
    StatusCode::OK
}

async fn logout() -> impl IntoResponse {
    StatusCode::OK
}

async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK
}

async fn verify_token() -> impl IntoResponse {
    StatusCode::OK
}