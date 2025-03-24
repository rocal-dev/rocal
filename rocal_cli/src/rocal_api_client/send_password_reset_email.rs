use serde::Serialize;

#[derive(Serialize)]
pub struct SendPasswordResetEmail {
    email: String,
}

impl SendPasswordResetEmail {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}
