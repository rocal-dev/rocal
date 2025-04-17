use serde::Deserialize;

#[derive(Deserialize)]
pub struct Product {
    id: u32,
    name: String,
    price: f64,
}

impl Product {
    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_price(&self) -> &f64 {
        &self.price
    }
}
