use serde::{Serialize, Deserialize};
use super::{RiskSignalDocumentStatus, RiskSignalDocumentType, RiskSignalFileType};
///Object containing metadata for the document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSignalDocumentReference {
    ///An identifier of the document referenced by the document metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    ///The name of the document
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_name: Option<String>,
    ///Type of a document for risk signal analysis
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_type: Option<RiskSignalDocumentType>,
    ///The file type for risk signal analysis
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_type: Option<RiskSignalFileType>,
    ///Status of a document for risk signal analysis
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RiskSignalDocumentStatus>,
}
impl std::fmt::Display for RiskSignalDocumentReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
