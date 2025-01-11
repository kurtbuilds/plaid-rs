use serde::{Serialize, Deserialize};
use super::{
    ForecastedMonthlyIncome, HistoricalAnnualIncome, IncomeSourcesCounts,
    MonitoringIncomeSource, TotalMonthlyIncomeInsights,
};
///An object representing the income subcategory of the report
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringIncomeInsights {
    ///An object representing the predicted average monthly net income amount. This amount reflects the funds deposited into the account and may not include any withheld income such as taxes or other payroll deductions
    pub forecasted_monthly_income: ForecastedMonthlyIncome,
    ///An object representing the historical annual income amount.
    pub historical_annual_income: HistoricalAnnualIncome,
    ///The income sources for this Item. Each entry in the array is a single income source
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub income_sources: Vec<MonitoringIncomeSource>,
    ///Details about the number of income sources
    pub income_sources_counts: IncomeSourcesCounts,
    ///Details about about the total monthly income
    pub total_monthly_income: TotalMonthlyIncomeInsights,
}
impl std::fmt::Display for MonitoringIncomeInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
