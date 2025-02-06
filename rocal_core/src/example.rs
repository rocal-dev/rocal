#[derive(Debug)]
pub struct Example {
    name: String,
}

impl Example {
    pub fn new(name: String) -> Self {
        Example { name }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
