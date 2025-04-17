use crate::models::{cart_item::CartItem, product::Product};

pub struct RootViewModel {
    products: Vec<Product>,
    cart_items: Vec<CartItem>,
}

impl RootViewModel {
    pub fn new(products: Vec<Product>, cart_items: Vec<CartItem>) -> Self {
        Self {
            products,
            cart_items,
        }
    }

    pub fn get_products(&self) -> &Vec<Product> {
        &self.products
    }

    pub fn get_cart_items(&self) -> &Vec<CartItem> {
        &self.cart_items
    }

    pub fn get_total_price(&self) -> f64 {
        let mut total: f64 = 0.0;

        for item in &self.cart_items {
            total += item.get_product_price();
        }

        total
    }
}
