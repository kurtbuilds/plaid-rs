use serde::{Serialize, Deserialize};
///The status of the delivery of the Hosted Link to the user
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinkDeliveryWebhookDeliveryStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
}
