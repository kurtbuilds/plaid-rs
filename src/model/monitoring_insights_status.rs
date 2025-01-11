use serde::{Serialize, Deserialize};
///Enum for the status of the insights
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MonitoringInsightsStatus {
    #[serde(rename = "AVAILABLE")]
    Available,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "UNSUPPORTED")]
    Unsupported,
    #[serde(rename = "UNHEALTHY")]
    Unhealthy,
}
