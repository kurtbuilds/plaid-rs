use serde::{Serialize, Deserialize};
use super::{
    BaseReportAverageFlowInsights, BaseReportLongestGapInsights,
    BaseReportNumberFlowInsights,
};
///Calculated insights derived from transaction-level data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAccountInsights {
    ///Average number of days between sequential transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_days_between_transactions: Option<f64>,
    ///Deprecated; use `average_inflow_amounts` instead. Average amount of debit transactions into the account. This array will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_inflow_amount: Option<Vec<BaseReportAverageFlowInsights>>,
    ///Customers must transition from `average_inflow_amount` by January 31st 2025. Average amount of debit transactions into the account in a time period. This array will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_inflow_amounts: Option<Vec<BaseReportAverageFlowInsights>>,
    ///Deprecated; use `average_outflow_amounts` instead. Average amount of transactions out of the account. This array will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_outflow_amount: Option<Vec<BaseReportAverageFlowInsights>>,
    ///Customers must transition from `average_outflow_amount` by January 31st 2025. Average amount of transactions out of the account in a time period. This array will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_outflow_amounts: Option<Vec<BaseReportAverageFlowInsights>>,
    ///Number of days days available in the base report for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_available: Option<i64>,
    ///Deprecated; use `longest_gaps_between_transactions` instead. Longest gap between sequential transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longest_gap_between_transactions: Option<Vec<BaseReportLongestGapInsights>>,
    ///Customers must transition from `longest_gap_between_transactions` by January 31st 2025. Longest gap between sequential transactions in a time period. This array can include multiple time periods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longest_gaps_between_transactions: Option<Vec<BaseReportLongestGapInsights>>,
    ///Date of the most recent transaction in the base report for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub most_recent_transaction_date: Option<chrono::NaiveDate>,
    ///Number of days with no transactions
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_days_no_transactions: Option<i64>,
    ///The number of debits into the account. This array will be empty for non-depository accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_inflows: Option<Vec<BaseReportNumberFlowInsights>>,
    ///The number of outflows from the account. This array will be empty for non-depository accounts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_outflows: Option<Vec<BaseReportNumberFlowInsights>>,
    ///Date of the earliest transaction in the base report for the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_transaction_date: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for BaseReportAccountInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
