use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RegisteredSyncServer {
    app_id: String,
    endpoint: String,
}

impl RegisteredSyncServer {
    pub fn get_app_id(&self) -> &str {
        &self.app_id
    }

    pub fn get_endpoint(&self) -> &str {
        &self.endpoint
    }
}
