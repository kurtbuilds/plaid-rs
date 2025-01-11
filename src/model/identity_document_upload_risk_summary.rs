use serde::{Serialize, Deserialize};
///Risk summary of an uploaded document.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityDocumentUploadRiskSummary {
    ///A number between 0 and 100, inclusive, where a score closer to 0 indicates a document is likely to be trustworthy and a score closer to 100 indicates a document is likely to be fraudulent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_score: Option<i64>,
}
impl std::fmt::Display for IdentityDocumentUploadRiskSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
