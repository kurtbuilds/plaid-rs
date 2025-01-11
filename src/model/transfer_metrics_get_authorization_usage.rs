use serde::{Serialize, Deserialize};
///Details regarding authorization usage.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferMetricsGetAuthorizationUsage {
    ///The daily credit utilization formatted as a decimal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_credit_utilization: Option<String>,
    ///The daily debit utilization formatted as a decimal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_debit_utilization: Option<String>,
    ///The monthly credit utilization formatted as a decimal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monthly_credit_utilization: Option<String>,
    ///The monthly debit utilization formatted as a decimal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monthly_debit_utilization: Option<String>,
}
impl std::fmt::Display for TransferMetricsGetAuthorizationUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
