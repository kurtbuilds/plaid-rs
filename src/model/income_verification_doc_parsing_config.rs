use serde::{Serialize, Deserialize};
///Analysis options to enable for document parsing
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IncomeVerificationDocParsingConfig {
    #[serde(rename = "ocr")]
    Ocr,
    #[serde(rename = "risk_signals")]
    RiskSignals,
}
