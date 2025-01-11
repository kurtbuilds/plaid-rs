use serde::{Serialize, Deserialize};
///Type of Notification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FdxNotificationType {
    #[serde(rename = "CONSENT_REVOKED")]
    ConsentRevoked,
    #[serde(rename = "CONSENT_UPDATED")]
    ConsentUpdated,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "SERVICE")]
    Service,
    #[serde(rename = "BALANCE")]
    Balance,
    #[serde(rename = "PLANNED_OUTAGE")]
    PlannedOutage,
}
