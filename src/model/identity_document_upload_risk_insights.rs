use serde::{Serialize, Deserialize};
use super::{IdentityDocumentUploadRiskSignal, IdentityDocumentUploadRiskSummary};
///Object representing fraud risk data of the uploaded document. Only provided when using Identity Document Upload with Fraud Risk enabled.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityDocumentUploadRiskInsights {
    ///An array of risk signals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_signals: Option<Vec<IdentityDocumentUploadRiskSignal>>,
    ///Risk summary of an uploaded document.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_summary: Option<IdentityDocumentUploadRiskSummary>,
}
impl std::fmt::Display for IdentityDocumentUploadRiskInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
