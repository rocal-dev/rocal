use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct OobCodeResponse {
    email: String,
}

impl OobCodeResponse {
    pub fn get_email(&self) -> &str {
        &self.email
    }
}
