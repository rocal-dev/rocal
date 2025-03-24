use serde::Serialize;

#[derive(Serialize)]
pub struct UserRefreshToken {
    refresh_token: String,
}

impl UserRefreshToken {
    pub fn new(token: &str) -> Self {
        Self {
            refresh_token: token.to_string(),
        }
    }
}
