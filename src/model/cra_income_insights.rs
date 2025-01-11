use serde::{Serialize, Deserialize};
use super::{CraBankIncomeItem, CraBankIncomeSummary, CraBankIncomeWarning};
///The Check Income Insights Report for an end user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraIncomeInsights {
    ///Summary for income across all income sources and items (max history of 730 days).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_income_summary: Option<CraBankIncomeSummary>,
    ///The number of days requested by the customer for the Check Income Insights Report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_requested: Option<i64>,
    ///The time when the Check Income Insights Report was generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_time: Option<chrono::DateTime<chrono::Utc>>,
    ///The list of Items in the report along with the associated metadata about the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CraBankIncomeItem>>,
    ///The unique identifier associated with the Check Income Insights Report.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    ///If data from the report was unable to be retrieved, the warnings object will contain information about the error that caused the data to be incomplete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<CraBankIncomeWarning>>,
}
impl std::fmt::Display for CraIncomeInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
