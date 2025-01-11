use serde::{Serialize, Deserialize};
///Details about about the total monthly income
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TotalMonthlyIncomeInsights {
    ///The aggregated income for the 30 days prior to subscription date
    pub baseline_amount: f64,
    ///The aggregated income of the last 30 days
    pub current_amount: f64,
}
impl std::fmt::Display for TotalMonthlyIncomeInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
