use serde::{Serialize, Deserialize};
///The type of Link callback event
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinkDeliveryWebhookCallbackType {
    #[serde(rename = "ON_SUCCESS")]
    OnSuccess,
    #[serde(rename = "ON_EVENT")]
    OnEvent,
    #[serde(rename = "ON_EXIT")]
    OnExit,
}
