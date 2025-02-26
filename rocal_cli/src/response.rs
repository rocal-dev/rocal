use serde::Deserialize;

#[derive(Deserialize)]
pub struct ResponseWithMessage<T>
where
    T: Clone,
{
    data: Option<T>,
    message: String,
}

impl<T> ResponseWithMessage<T>
where
    T: Clone,
{
    pub fn get_data(&self) -> &Option<T> {
        &self.data
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }
}
