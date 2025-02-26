use serde::Serialize;

#[derive(Serialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

impl LoginUser {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}
