use serde::{Serialize, Deserialize};
///Documentation not found in the MISMO model viewer and not provided by Freddie Mac.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServiceProductFulfillmentIdentifier {
    #[serde(rename = "VOA")]
    Voa,
    #[serde(rename = "VOE")]
    Voe,
}
