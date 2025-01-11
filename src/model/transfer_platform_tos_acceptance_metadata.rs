use serde::{Serialize, Deserialize};
///Metadata related to the acceptance of Terms of Service
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferPlatformTosAcceptanceMetadata {
    ///Indicates whether the TOS agreement was accepted
    pub agreement_accepted: bool,
    ///ISO8601 timestamp indicating when the originator accepted the TOS
    pub agreement_accepted_at: chrono::DateTime<chrono::Utc>,
    ///The IP address of the originator when they accepted the TOS. Formatted as an IPv4 or IPv6 IP address
    pub originator_ip_address: String,
}
impl std::fmt::Display for TransferPlatformTosAcceptanceMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
