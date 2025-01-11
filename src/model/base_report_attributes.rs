use serde::{Serialize, Deserialize};
use super::{
    TotalInflowAmount, TotalInflowAmount30D, TotalInflowAmount60D, TotalInflowAmount90D,
    TotalOutflowAmount, TotalOutflowAmount30D, TotalOutflowAmount60D,
    TotalOutflowAmount90D,
};
///Calculated attributes derived from transaction-level data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BaseReportAttributes {
    ///Prediction indicator of whether the account is a primary account. Only one account per account type across the items connected will have a value of true.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary_account: Option<bool>,
    ///The number of NSF and overdraft fee transactions in the time range for the report in the given account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsf_overdraft_transactions_count: Option<i64>,
    ///Value ranging from 0-1. The higher the score, the more confident we are of the account being the primary account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_account_score: Option<f64>,
    ///Total amount of debit transactions into the account in the time period of the report. This field will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_inflow_amount: Option<TotalInflowAmount>,
    ///Total amount of debit transactions into the account in the last 30 days. This field will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "total_inflow_amount_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_inflow_amount30_d: Option<TotalInflowAmount30D>,
    ///Total amount of debit transactions into the account in the last 60 days. This field will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "total_inflow_amount_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_inflow_amount60_d: Option<TotalInflowAmount60D>,
    ///Total amount of debit transactions into the account in the last 90 days. This field will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "total_inflow_amount_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_inflow_amount90_d: Option<TotalInflowAmount90D>,
    ///Total amount of credit transactions into the account in the time period of the report. This field will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_outflow_amount: Option<TotalOutflowAmount>,
    ///Total amount of credit transactions into the account in the last 30 days. This field will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "total_outflow_amount_30d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_outflow_amount30_d: Option<TotalOutflowAmount30D>,
    ///Total amount of credit transactions into the account in the last 60 days. This field will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "total_outflow_amount_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_outflow_amount60_d: Option<TotalOutflowAmount60D>,
    ///Total amount of credit transactions into the account in the last 90 days. This field will be empty for non-depository accounts. This field only takes into account USD transactions from the account.
    #[serde(rename = "total_outflow_amount_90d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_outflow_amount90_d: Option<TotalOutflowAmount90D>,
}
impl std::fmt::Display for BaseReportAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
