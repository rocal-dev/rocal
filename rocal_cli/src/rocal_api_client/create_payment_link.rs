use serde::Serialize;

#[derive(Serialize)]
pub struct CreatePaymentLink {
    plan: String,
}

impl CreatePaymentLink {
    pub fn new(plan: &str) -> Self {
        Self {
            plan: plan.to_string(),
        }
    }
}
