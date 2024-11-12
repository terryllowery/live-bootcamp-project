use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env as std_env;

pub const JWT_COOKIE_NAME: &str = "jwt";

lazy_static! {
    pub static ref JWT_SECRET: String = set_token();
}

// This is definitely NOT a good secret. We will update it soon!
// const JWT_SECRET: &str = "secret";

fn set_token() -> String {
    dotenv().ok();
    let secret = std_env::var("JWT_SECRET").expect("JWT_SECRET_ENV_VAR must be set");

    if secret.is_empty() {
        panic!("JWT_SECRET_ENV_VAR must be set");
    }
    secret
}

pub mod env {
    
}