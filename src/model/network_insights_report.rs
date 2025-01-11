use serde::{Serialize, Deserialize};
use super::{NetworkInsightsItem, NetworkInsightsSchema};
///Contains data for the Network Insights Report.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkInsightsReport {
    ///The time when the Network Insights Report was generated.
    pub generated_time: chrono::DateTime<chrono::Utc>,
    ///A list of Items associated with the provided access_tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NetworkInsightsItem>,
    ///A map of network attributes, where the key is a string, and the value is a float, int, or boolean.
    pub network_attributes: NetworkInsightsSchema,
    ///The unique identifier associated with the Network Insights report object.
    pub report_id: String,
}
impl std::fmt::Display for NetworkInsightsReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
