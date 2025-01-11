use serde::{Serialize, Deserialize};
use super::{CraNetworkInsightsItem, NetworkInsightsSchema};
///Contains data for the CRA Network Attributes Report.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraNetworkInsightsReport {
    ///The time when the Network Attributes Report was generated.
    pub generated_time: chrono::DateTime<chrono::Utc>,
    ///The Items the end user connected in Link.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CraNetworkInsightsItem>,
    ///A map of network attributes, where the key is a string, and the value is a float, int, or boolean.
    pub network_attributes: NetworkInsightsSchema,
    ///The unique identifier associated with the Network Attributes report object.
    pub report_id: String,
}
impl std::fmt::Display for CraNetworkInsightsReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
