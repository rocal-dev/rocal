use crate::{
    models::{sales::Sales, sales_item::SalesItem, sales_log::SalesLog},
    Database,
};
use std::sync::Arc;

pub struct SalesRepository {
    database: Arc<Database>,
}

impl SalesRepository {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn get_all(&self) -> Result<Vec<SalesLog>, Option<String>> {
        let result: Vec<SalesLog> = self
            .database
            .query("select id, created_at from sales order by created_at desc;")
            .fetch()
            .await
            .map_err(|err| err.as_string())?;

        Ok(result)
    }

    pub async fn get_all_items(&self, id: u32) -> Result<Vec<SalesItem>, Option<String>> {
        let result: Vec<SalesItem> = self
            .database
            .query(&format!(
                r#"
                select product_name, product_price, number_of_items from sales_items where sales_id = {}
                "#,
                id
            ))
            .fetch()
            .await
            .map_err(|err| err.as_string())?;

        Ok(result)
    }

    pub async fn create(&self, sales_list: Vec<Sales>) -> Result<(), Option<String>> {
        let mut values: Vec<String> = vec![];

        for sales in sales_list {
            values.push(format!(
                "((select sales_id from sid), {}, '{}', {}, {})",
                sales.get_product_id(),
                sales.get_product_name(),
                sales.get_product_price(),
                sales.get_number_of_items()
            ))
        }

        let values = values.join(",");

        self.database
            .query(&format!(
                r#"
                  begin immediate;
                    insert into sales default values;

                    with sid as (
                      select last_insert_rowid() as sales_id
                    )
                    insert into
                      sales_items (sales_id, product_id, product_name, product_price, number_of_items)
                    values
                      {};
                  commit;
                "#,
                values
            ))
            .execute()
            .await
            .map_err(|err| err.as_string())?;

        Ok(())
    }
}
