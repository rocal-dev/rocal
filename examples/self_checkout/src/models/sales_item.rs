use serde::Deserialize;

#[derive(Deserialize)]
pub struct SalesItem {
    product_name: String,
    product_price: f64,
    number_of_items: u32,
}

impl SalesItem {
    pub fn get_product_name(&self) -> &str {
        &self.product_name
    }

    pub fn get_product_price(&self) -> &f64 {
        &self.product_price
    }

    pub fn get_number_of_items(&self) -> &u32 {
        &self.number_of_items
    }
}
