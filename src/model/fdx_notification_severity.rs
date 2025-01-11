use serde::{Serialize, Deserialize};
///Severity level of notification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FdxNotificationSeverity {
    #[serde(rename = "EMERGENCY")]
    Emergency,
    #[serde(rename = "ALERT")]
    Alert,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "NOTICE")]
    Notice,
    #[serde(rename = "INFO")]
    Info,
}
