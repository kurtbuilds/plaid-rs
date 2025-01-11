use serde::{Serialize, Deserialize};
///Recipient metadata fields that are defined by FDX
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FdxRecipientMetadata {
    ///The recipient name displayed by the Data Provider during the consent flow
    pub client_name: String,
    ///Data Recipient Logo URL location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    ///The recipient identifier
    pub recipient_id: String,
    ///The legal name of the recipient
    pub third_party_legal_name: String,
}
impl std::fmt::Display for FdxRecipientMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
