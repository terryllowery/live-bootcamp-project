use auth_service::Application;

#[tokio::main]
async fn main() {
    // Here we are using ip 0.0.0.0 so the service is listening on all the configured network interfaces.
    // This is needed for Docker to work, which we will add later on.
    // See: https://stackoverflow.com/questions/39525820/docker-port-forwarding-not-working
    let address = "0.0.0.0:3000";

    let app = Application::build(address)
        .await
        .expect("Application failed to build");

    app.run().await.expect("Application failed to run");
}
