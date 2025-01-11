use serde::{Serialize, Deserialize};
///A broad categorization of the consent event.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConsentEventType {
    #[serde(rename = "CONSENT_GRANTED")]
    ConsentGranted,
    #[serde(rename = "CONSENT_REVOKED")]
    ConsentRevoked,
    #[serde(rename = "CONSENT_UPDATED")]
    ConsentUpdated,
}
