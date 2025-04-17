use crate::models::sales_log::SalesLog;

pub struct SalesLogViewModel {
    sales_logs: Vec<SalesLog>,
}

impl SalesLogViewModel {
    pub fn new(sales_logs: Vec<SalesLog>) -> Self {
        Self { sales_logs }
    }

    pub fn get_sales_logs(&self) -> &Vec<SalesLog> {
        &self.sales_logs
    }
}
