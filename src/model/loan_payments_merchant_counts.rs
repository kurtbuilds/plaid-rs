use serde::{Serialize, Deserialize};
///Details regarding the number of unique loan payment merchants
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoanPaymentsMerchantCounts {
    ///The number of unique loan payment merchants detected in the 30 days before the subscription date
    pub baseline_count: f64,
    ///The current number of unique loan payment merchants detected in the last 30 days
    pub current_count: f64,
}
impl std::fmt::Display for LoanPaymentsMerchantCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
