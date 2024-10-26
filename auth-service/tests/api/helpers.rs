use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    Application,
    app_state::AppState,
    services::hashmap_user_store::HashmapUserStore,
};
use reqwest;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {

    pub async fn new() -> Self {
        let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let app_state = AppState::new(user_store);



        let app = Application::build("127.0.0.1:0", app_state)
            .await
            .expect("Failed to build application");

        let address = format!("http://{}", app.address.clone());

        // Run authservice in a separate async task
        // to avoid blocking the main test thread
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        // Create a new TestApp instance and return it
        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }


    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
    
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn login(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
