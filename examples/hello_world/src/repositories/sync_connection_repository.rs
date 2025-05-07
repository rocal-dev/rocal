use std::sync::Arc;

use wasm_bindgen::JsValue;

use crate::{models::sync_connection::SyncConnection, Database};

pub struct SyncConnectionRepository {
    database: Arc<Database>,
}

impl SyncConnectionRepository {
    pub fn new(database: Arc<Database>) -> Self {
        SyncConnectionRepository { database }
    }

    pub async fn get(&self) -> Result<Option<SyncConnection>, JsValue> {
        let mut result: Vec<SyncConnection> = self
            .database
            .query("select id from sync_connections limit 1;")
            .fetch()
            .await?;

        match result.pop() {
            Some(conn) => Ok(Some(SyncConnection::new(conn.get_id().to_string()))),
            None => Ok(None),
        }
    }

    pub async fn create(&self, id: &str, password: &str) -> Result<(), JsValue> {
        match self
            .database
            .query(&format!(
                "insert into sync_connections (id, password) values ('{}', '{}')",
                id, password
            ))
            .execute()
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
