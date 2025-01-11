use serde::{Serialize, Deserialize};
use super::{MonitoringIncomeInsights, MonitoringLoanInsights};
///An object representing the Monitoring Insights for the given Item
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringInsights {
    ///An object representing the income subcategory of the report
    pub income: MonitoringIncomeInsights,
    ///An object representing the loan exposure subcategory of the report
    pub loans: MonitoringLoanInsights,
}
impl std::fmt::Display for MonitoringInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
