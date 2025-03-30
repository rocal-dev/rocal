use super::utils::{get_user_input, get_user_secure_input};
use crate::{
    rocal_api_client::{
        create_user::CreateUser, send_email_verification::SendEmailVerification, RocalAPIClient,
    },
    token_manager::{Kind, TokenManager},
};

pub async fn register() {
    let email = get_user_input("your email");
    let mut password = get_user_secure_input("password");
    let mut confirm_password = get_user_secure_input("confirm password");

    while password != confirm_password {
        println!("The password should be same as the confirm password");

        password = get_user_secure_input("password");
        confirm_password = get_user_secure_input("confirm password");
    }

    let workspace = get_user_input("a workspace name");

    let client = RocalAPIClient::new();
    let user = CreateUser::new(&email, &password, &workspace);

    if let Err(err) = client.sign_up(user).await {
        eprintln!("{}", err);
        return;
    }

    if let Ok(token) = TokenManager::get_token(Kind::RocalAccessToken) {
        let req = SendEmailVerification::new(&token);
        client.send_email_verification(req).await;
    }
}
