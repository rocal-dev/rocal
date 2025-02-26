use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct UserLoginToken {
    id_token: String,
    refresh_token: String,
    expires_in: String,
    local_id: String,
}

impl UserLoginToken {
    pub fn get_id_token(&self) -> &str {
        &self.id_token
    }

    pub fn get_refresh_token(&self) -> &str {
        &self.refresh_token
    }

    pub fn get_expires_in(&self) -> &str {
        &self.expires_in
    }
}
