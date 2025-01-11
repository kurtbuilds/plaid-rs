use serde::{Serialize, Deserialize};
///Type of entity causing origination of a notification
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FdxNotificationPayloadIdType {
    #[serde(rename = "ACCOUNT")]
    Account,
    #[serde(rename = "CUSTOMER")]
    Customer,
    #[serde(rename = "PARTY")]
    Party,
    #[serde(rename = "MAINTENANCE")]
    Maintenance,
    #[serde(rename = "CONSENT")]
    Consent,
}
