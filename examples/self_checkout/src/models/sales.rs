use serde::Deserialize;

#[derive(Deserialize)]
pub struct Sales {
    product_id: u32,
    product_name: String,
    product_price: f64,
    number_of_items: u32,
}

impl Sales {
    pub fn new(
        product_id: u32,
        product_name: &str,
        product_price: f64,
        number_of_items: u32,
    ) -> Self {
        Self {
            product_id,
            product_name: product_name.to_string(),
            product_price,
            number_of_items,
        }
    }

    pub fn get_product_id(&self) -> &u32 {
        &self.product_id
    }

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
