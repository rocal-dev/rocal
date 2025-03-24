use super::utils::get_user_input;
use crate::{
    rocal_api_client::{
        create_user::CreateUser, send_email_verification::SendEmailVerification, RocalAPIClient,
    },
    token_manager::{Kind, TokenManager},
};

pub async fn register() {
    let email = get_user_input("your email");
    let password = get_user_input("a password");
    let workspace = get_user_input("a workspace name");

    let client = RocalAPIClient::new();
    let user = CreateUser::new(&email, &password, &workspace);

    client.sign_up(user).await;

    if let Ok(token) = TokenManager::get_token(Kind::RocalAccessToken) {
        let req = SendEmailVerification::new(&token);
        client.send_email_verification(req).await;
    }
}
