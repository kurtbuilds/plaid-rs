use serde::{Serialize, Deserialize};
///Details regarding return rates.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferMetricsGetReturnRatesOverInterval {
    ///The administrative return rate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_return_rate: Option<String>,
    ///The overall return rate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overall_return_rate: Option<String>,
    ///The unauthorized return rate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unauthorized_return_rate: Option<String>,
}
impl std::fmt::Display for TransferMetricsGetReturnRatesOverInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
