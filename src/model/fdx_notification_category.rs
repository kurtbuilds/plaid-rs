use serde::{Serialize, Deserialize};
///Category of Notification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FdxNotificationCategory {
    #[serde(rename = "SECURITY")]
    Security,
    #[serde(rename = "MAINTENANCE")]
    Maintenance,
    #[serde(rename = "FRAUD")]
    Fraud,
    #[serde(rename = "CONSENT")]
    Consent,
    #[serde(rename = "NEW_DATA")]
    NewData,
}
