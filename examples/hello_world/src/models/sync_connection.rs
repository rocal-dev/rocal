use serde::Deserialize;

#[derive(Deserialize)]
pub struct SyncConnection {
    id: String,
}

impl SyncConnection {
    pub fn new(id: String) -> Self {
        SyncConnection { id }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}
