pub struct Error<T> {
    data: Option<T>,
    message: String,
}

impl<T> Error<T> {
    pub fn new() -> Self {
        Self {
            data: None,
            message: "".to_string(),
        }
    }

    pub fn set_data(&mut self, data: T) {
        self.data = Some(data);
    }

    pub fn set_message(&mut self, message: &str) {
        self.message = message.to_string();
    }

    pub fn get_data(&self) -> &Option<T> {
        &self.data
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}
