use serde::{Serialize, Deserialize};
///Enum for the status of the Item's insights
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MonitoringItemStatusCode {
    #[serde(rename = "AVAILABLE")]
    Available,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "ITEM_NOT_SUPPORTED")]
    ItemNotSupported,
    #[serde(rename = "ITEM_LOGIN_REQUIRED")]
    ItemLoginRequired,
}
