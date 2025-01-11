use serde::{Serialize, Deserialize};
///The communication method used to deliver the Hosted Link session
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinkDeliveryWebhookCommunicationMethod {
    #[serde(rename = "SMS")]
    Sms,
    #[serde(rename = "EMAIL")]
    Email,
}
