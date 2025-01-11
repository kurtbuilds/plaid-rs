use serde::{Serialize, Deserialize};
///Status of a document for risk signal analysis
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RiskSignalDocumentStatus {
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "PROCESSING_COMPLETE")]
    ProcessingComplete,
    #[serde(rename = "PROCESSING_ERROR")]
    ProcessingError,
    #[serde(rename = "PASSWORD_PROTECTED")]
    PasswordProtected,
    #[serde(rename = "VIRUS_DETECTED")]
    VirusDetected,
}
