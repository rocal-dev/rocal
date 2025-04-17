use crate::{models::cart_item::CartItem, Database};
use std::sync::Arc;

pub struct CartRepository {
    database: Arc<Database>,
}

impl CartRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn get_all_items(&self) -> Result<Vec<CartItem>, Option<String>> {
        let result: Vec<CartItem> = self
            .database
            .query(
                r#"
               select
                 c.id as id,
                 c.product_id as product_id,
                 p.name as product_name,
                 p.price as product_price,
                 c.number_of_items as number_of_items
               from cart_items as c
               inner join products as p on c.product_id = p.id;
            "#,
            )
            .await
            .map_err(|err| err.as_string())?;

        Ok(result)
    }

    pub async fn add_item(&self, product_id: u32) -> Result<(), Option<String>> {
        let mut items: Vec<CartItem> = self
            .database
            .query(&format!(
                r#"
               select
                 c.id as id,
                 c.product_id as product_id,
                 p.name as product_name,
                 p.price as product_price,
                 c.number_of_items as number_of_items
               from cart_items as c
               inner join products as p on c.product_id = p.id
               where p.id = {} limit 1;"#,
                product_id
            ))
            .await
            .map_err(|err| err.as_string())?;

        match items.pop() {
            Some(item) => {
                let number_of_items = item.get_number_of_items() + 1;
                self.database
                    .exec(&format!(
                        "update cart_items set number_of_items = {} where product_id = {}",
                        number_of_items, product_id
                    ))
                    .await
                    .map_err(|err| err.as_string())?;
            }
            None => {
                let number_of_items = 1;
                self.database
                    .exec(&format!(
                        "insert into cart_items (product_id, number_of_items) values ({}, {})",
                        product_id, number_of_items
                    ))
                    .await
                    .map_err(|err| err.as_string())?;
            }
        };

        Ok(())
    }

    pub async fn remove_item(&self, product_id: u32) -> Result<(), Option<String>> {
        self.database
            .exec(&format!(
                "delete from cart_items where product_id = {}",
                product_id
            ))
            .await
            .map_err(|err| err.as_string())?;

        Ok(())
    }

    pub async fn remove_all_items(&self) -> Result<(), Option<String>> {
        self.database
            .exec("delete from cart_items;")
            .await
            .map_err(|err| err.as_string())?;

        Ok(())
    }
}
