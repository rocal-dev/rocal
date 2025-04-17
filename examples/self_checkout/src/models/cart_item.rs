use serde::Deserialize;

#[derive(Deserialize)]
pub struct CartItem {
    id: u32,
    product_id: u32,
    product_name: String,
    product_price: f64,
    number_of_items: u32,
}

impl CartItem {
    pub fn get_product_id(&self) -> &u32 {
        &self.product_id
    }

    pub fn get_product_name(&self) -> &str {
        &self.product_name
    }

    pub fn get_product_price(&self) -> f64 {
        self.product_price * self.number_of_items as f64
    }

    pub fn get_number_of_items(&self) -> &u32 {
        &self.number_of_items
    }
}
