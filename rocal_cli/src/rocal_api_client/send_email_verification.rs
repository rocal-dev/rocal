use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailVerification {
    id_token: String,
}

impl SendEmailVerification {
    pub fn new(id_token: &str) -> Self {
        Self {
            id_token: id_token.to_string(),
        }
    }
}
