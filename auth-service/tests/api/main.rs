mod helpers;
mod login;
mod logout;
mod root;
mod routes;
mod signup;
mod verify_2fa;
mod verify_token;

use helpers::TestApp;

fn main() {
    login::login_returns_200();
    logout::logout_returns_200();
    root::root_returns_auth_ui();
    signup::should_return_422_if_malformed_request();
    verify_2fa::verify_2fa_returns_200();
    verify_token::verify_token_returns_200();
}
