use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct FlashMemory {
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl FlashMemory {
    pub fn new(data: Arc<Mutex<HashMap<String, String>>>) -> Self {
        Self { data }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        let cloned = self.data.clone();

        {
            let mut guard = cloned.lock().map_err(|err| err.to_string())?;
            guard.insert(key.to_string(), value.to_string());
        }

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        let cloned = self.data.clone();
        let result = {
            let mut guard = cloned.lock().map_err(|err| err.to_string())?;
            let value = guard.get(key).cloned();
            guard.remove(key);
            value
        };
        Ok(result.unwrap_or(String::from("")))
    }
}
