use serde::{Serialize, Deserialize};
use super::{IdentityDocumentUploadMetadata, IdentityDocumentUploadRiskInsights};
///Document object with metadata of the uploaded document
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityDocumentUpload {
    ///A UUID identifying the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///Metadata pertaining to the document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<IdentityDocumentUploadMetadata>,
    ///Object representing fraud risk data of the uploaded document. Only provided when using Identity Document Upload with Fraud Risk enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_insights: Option<IdentityDocumentUploadRiskInsights>,
}
impl std::fmt::Display for IdentityDocumentUpload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
