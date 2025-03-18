use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Subdomain {
    app_name: String,
    subdomain: String,
}

impl Subdomain {
    pub fn get_subdomain(&self) -> &str {
        &self.subdomain
    }
}
