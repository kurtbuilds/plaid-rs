use serde::{Serialize, Deserialize};
///Details regarding the number of loan payments
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoanPaymentsCounts {
    ///The number of loan payments made in the 30 days before the subscription date
    pub baseline_count: f64,
    ///The current number of loan payments made in the last 30 days
    pub current_count: f64,
}
impl std::fmt::Display for LoanPaymentsCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
