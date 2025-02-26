use crate::rocal_api_client::{send_password_reset_email::SendPasswordResetEmail, RocalAPIClient};

use super::utils::get_user_input;

pub async fn reset() {
    let email = get_user_input("your email");

    let client = RocalAPIClient::new();
    let req = SendPasswordResetEmail::new(&email);

    if let Err(err) = client.send_password_reset_email(req).await {
        match err.as_str() {
            "INVALID_LOGIN_CREDENTIALS" => eprintln!("Your email address or password is wrong"),
            "INVALID_EMAIL" => eprintln!("An email address you entered is invalid"),
            _ => eprintln!("{}", err),
        }
    }
}
