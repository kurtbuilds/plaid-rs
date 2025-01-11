use serde::{Serialize, Deserialize};
use super::CraMonitoringInsightsItem;
///CraMonitoringInsightsGetResponse defines the response schema for `cra/monitoring_insights/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraMonitoringInsightsGetResponse {
    ///An array of Monitoring Insights Items associated with the user.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CraMonitoringInsightsItem>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for CraMonitoringInsightsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
