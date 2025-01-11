use serde::{Serialize, Deserialize};
///An object representing the historical annual income amount.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoricalAnnualIncome {
    ///The historical annual income at the time of subscription
    pub baseline_amount: f64,
    ///The current historical annual income
    pub current_amount: f64,
}
impl std::fmt::Display for HistoricalAnnualIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
