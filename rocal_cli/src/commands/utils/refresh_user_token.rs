use crate::{
    commands::utils::color::Color,
    rocal_api_client::RocalAPIClient,
    token_manager::{Kind, TokenManager},
};

pub async fn refresh_user_token() {
    if let Ok(refresh_token) = TokenManager::get_token(Kind::RocalRefreshToken) {
        let client = RocalAPIClient::new();
        if let Err(err) = client.refresh_user_login_token(&refresh_token).await {
            println!("{}", Color::Red.text(&err));
        }
    }
}
