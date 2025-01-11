use serde::{Serialize, Deserialize};
use super::{AccountIdentityDocumentUpload, Item};
///IdentityDocumentsUploadsGetResponse defines the response schema for `/identity/documents/uploads/get`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityDocumentsUploadsGetResponse {
    ///The accounts for which Identity data has been requested
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<AccountIdentityDocumentUpload>,
    ///Metadata about the Item.
    pub item: Item,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for IdentityDocumentsUploadsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
