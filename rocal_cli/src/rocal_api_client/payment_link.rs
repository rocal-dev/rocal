use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct PaymentLink {
    url: String,
}

impl PaymentLink {
    pub fn get_url(&self) -> &str {
        &self.url
    }
}
