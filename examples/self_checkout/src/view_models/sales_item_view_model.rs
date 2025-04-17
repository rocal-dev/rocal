use crate::models::sales_item::SalesItem;

pub struct SalesItemViewModel {
    sales_items: Vec<SalesItem>,
}

impl SalesItemViewModel {
    pub fn new(sales_items: Vec<SalesItem>) -> Self {
        Self { sales_items }
    }

    pub fn get_sales_items(&self) -> &Vec<SalesItem> {
        &self.sales_items
    }

    pub fn get_total_price(&self) -> f64 {
        let mut total: f64 = 0.0;

        for item in &self.sales_items {
            total += item.get_product_price();
        }

        total
    }
}
