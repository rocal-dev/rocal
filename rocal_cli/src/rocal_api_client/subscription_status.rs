use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct SubscriptionStatus {
    plan: String,
    cancel_at_period_end: bool,
}

impl SubscriptionStatus {
    pub fn get_plan(&self) -> &str {
        &self.plan
    }

    pub fn get_cancel_at_period_end(&self) -> &bool {
        &self.cancel_at_period_end
    }
}
