use serde::{Serialize, Deserialize};
///Information specific to wire transfers.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferWireDetails {
    ///Additional information from the wire originator to the beneficiary. Max 140 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_to_beneficiary: Option<String>,
}
impl std::fmt::Display for TransferWireDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
