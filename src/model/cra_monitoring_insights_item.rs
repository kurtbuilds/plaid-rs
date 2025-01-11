use serde::{Serialize, Deserialize};
use super::{MonitoringInsights, MonitoringInsightsItemStatus};
///An object representing a Monitoring Insights Item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraMonitoringInsightsItem {
    ///The date and time when the specific insights were generated (per-item), in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (e.g. "2018-04-12T03:32:11Z").
    pub date_generated: chrono::DateTime<chrono::Utc>,
    ///An object representing the Monitoring Insights for the given Item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insights: Option<MonitoringInsights>,
    ///The `item_id` of the Item associated with the insights
    pub item_id: String,
    ///An object with details of the Monitoring Insights Item's status.
    pub status: MonitoringInsightsItemStatus,
}
impl std::fmt::Display for CraMonitoringInsightsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
