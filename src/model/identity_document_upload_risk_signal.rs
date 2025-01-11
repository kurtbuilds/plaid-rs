use serde::{Serialize, Deserialize};
///Risk signals tied to the document
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityDocumentUploadRiskSignal {
    ///Indicates whether fraud risk was detected on the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_fraud_risk: Option<bool>,
    ///The relevant page associated with the risk signal. If the risk signal is not associated with a specific page, the value will be 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
    ///A human-readable explanation providing more detail about the specific risk signal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signal_description: Option<String>,
    ///The type of risk found.
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for IdentityDocumentUploadRiskSignal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
