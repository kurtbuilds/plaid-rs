use serde::{Serialize, Deserialize};
///Priority of notification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FdxNotificationPriority {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "LOW")]
    Low,
}
