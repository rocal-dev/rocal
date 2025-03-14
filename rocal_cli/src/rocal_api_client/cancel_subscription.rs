use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct CancelSubscription {
    reason: String,
}

impl CancelSubscription {
    pub fn new(reason: u32) -> Result<Self, String> {
        let reasons = Self::get_reasons();

        if let Some(reason) = reasons.get(&reason) {
            Ok(Self {
                reason: reason.to_string(),
            })
        } else {
            Err("The reason number is out of options".to_string())
        }
    }

    pub fn get_reasons() -> HashMap<u32, String> {
        let mut reasons = HashMap::new();

        reasons.insert(1, "Custormer service was less than expected".to_string());
        reasons.insert(2, "Quality was less than expected".to_string());
        reasons.insert(3, "Some features are missing".to_string());
        reasons.insert(4, "I'm switching to a different service".to_string());
        reasons.insert(5, "Ease of use was less than expected".to_string());
        reasons.insert(6, "It's too expensive".to_string());
        reasons.insert(7, "I don't use the service enough".to_string());
        reasons.insert(8, "Other reason".to_string());

        reasons
    }
}
