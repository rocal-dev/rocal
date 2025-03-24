use super::utils::get_user_input;
use crate::rocal_api_client::{login_user::LoginUser, RocalAPIClient};

pub async fn login() {
    let email = get_user_input("your email");
    let password = get_user_input("your password");

    let client = RocalAPIClient::new();
    let user = LoginUser::new(&email, &password);

    if let Err(err) = client.sign_in(user).await {
        match err.as_str() {
            "INVALID_LOGIN_CREDENTIALS" => eprintln!("Your email address or password is wrong"),
            "INVALID_EMAIL" => eprintln!("An email address you entered is invalid"),
            _ => eprintln!("{}", err),
        }
    }
}
