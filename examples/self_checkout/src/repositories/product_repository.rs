use crate::{models::product::Product, Database};
use std::sync::Arc;

pub struct ProductRepository {
    database: Arc<Database>,
}

impl ProductRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn get_all(&self) -> Result<Vec<Product>, Option<String>> {
        let result: Vec<Product> = self
            .database
            .query("select id, name, price from products;")
            .await
            .map_err(|err| err.as_string())?;

        Ok(result)
    }
}
