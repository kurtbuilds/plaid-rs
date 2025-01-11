use serde::{Serialize, Deserialize};
use super::ExtendedRecipientMetadata;
///GetRecipientsResponse defines the response schema for `/fdx/recipients`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRecipientsResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipients: Vec<ExtendedRecipientMetadata>,
}
impl std::fmt::Display for GetRecipientsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
