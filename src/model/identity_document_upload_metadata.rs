use serde::{Serialize, Deserialize};
///Metadata pertaining to the document.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityDocumentUploadMetadata {
    ///The submitted document type. Currently, this will always be `BANK_STATEMENT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    ///Boolean field indicating whether the uploaded document's account number matches the account number we have on file. If `false`, it is not recommended to accept the uploaded identity data as accurate without further verification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_account_number_match: Option<bool>,
    ///The timestamp when the document was last updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    ///The number of pages in the uploaded document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_count: Option<i64>,
    ///The timestamp when the document was originally uploaded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for IdentityDocumentUploadMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
