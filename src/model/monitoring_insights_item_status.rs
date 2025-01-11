use serde::{Serialize, Deserialize};
use super::MonitoringItemStatusCode;
///An object with details of the Monitoring Insights Item's status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringInsightsItemStatus {
    /**A reason for why a Monitoring Insights Report is not available.
This field will only be populated when the `status_code` is not `AVAILABLE`*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    ///Enum for the status of the Item's insights
    pub status_code: MonitoringItemStatusCode,
}
impl std::fmt::Display for MonitoringInsightsItemStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
