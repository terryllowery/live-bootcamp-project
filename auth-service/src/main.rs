use std::sync::Arc;
use tokio::sync::RwLock;

use auth_service::{
    app_state::AppState, services::hashmap_user_store::HashmapUserStore, Application,
};

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let app_state = AppState::new(user_store);

    let address = "0.0.0.0:3000";

    let app = Application::build(address, app_state)
        .await
        .expect("Application failed to build");

    app.run().await.expect("Application failed to run");
}
