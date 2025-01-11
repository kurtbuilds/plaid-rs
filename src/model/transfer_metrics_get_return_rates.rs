use serde::{Serialize, Deserialize};
use super::TransferMetricsGetReturnRatesOverInterval;
///Details regarding return rates.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferMetricsGetReturnRates {
    ///Details regarding return rates.
    #[serde(rename = "last_60d")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last60_d: Option<TransferMetricsGetReturnRatesOverInterval>,
}
impl std::fmt::Display for TransferMetricsGetReturnRates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
