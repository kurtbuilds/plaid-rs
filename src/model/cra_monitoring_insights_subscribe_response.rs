use serde::{Serialize, Deserialize};
///CraMonitoringInsightsSubscribeResponse defines the response schema for `cra/monitoring_insights/subscribe`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraMonitoringInsightsSubscribeResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///A unique identifier for the subscription.
    pub subscription_id: String,
}
impl std::fmt::Display for CraMonitoringInsightsSubscribeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
