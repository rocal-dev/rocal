use serde::Serialize;

#[derive(Serialize)]
pub struct CreateUser {
    email: String,
    password: String,
    workspace: String,
}

impl CreateUser {
    pub fn new(email: &str, password: &str, workspace: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
            workspace: workspace.to_string(),
        }
    }
}
