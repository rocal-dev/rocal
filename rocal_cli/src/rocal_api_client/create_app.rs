use std::time::SystemTime;

use serde::Serialize;

#[derive(Serialize)]
pub struct CreateApp {
    app_name: String,
    subdomain: String,
    version: String,
}

impl CreateApp {
    pub fn new(app_name: &str, subdomain: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            subdomain: subdomain.to_string(),
            version: Self::default_version().to_string(),
        }
    }

    fn default_version() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
