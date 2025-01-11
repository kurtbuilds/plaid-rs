use serde::{Serialize, Deserialize};
/**The delivery method to be used to deliver the Hosted Link session URL.

`SMS`: The URL will be delivered through SMS

`EMAIL`: The URL will be delivered through email*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinkDeliveryDeliveryMethod {
    #[serde(rename = "SMS")]
    Sms,
    #[serde(rename = "EMAIL")]
    Email,
}
