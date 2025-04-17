use serde::Deserialize;

#[derive(Deserialize)]
pub struct SalesLog {
    id: u32,
    created_at: String,
}

impl SalesLog {
    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_created_at(&self) -> &str {
        &self.created_at
    }
}
