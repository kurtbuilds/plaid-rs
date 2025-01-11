use serde::{Serialize, Deserialize};
///An object representing the predicted average monthly net income amount. This amount reflects the funds deposited into the account and may not include any withheld income such as taxes or other payroll deductions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForecastedMonthlyIncome {
    ///The forecasted monthly income at the time of subscription
    pub baseline_amount: f64,
    ///The current forecasted monthly income
    pub current_amount: f64,
}
impl std::fmt::Display for ForecastedMonthlyIncome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
