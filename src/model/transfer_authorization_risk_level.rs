use serde::{Serialize, Deserialize};
///Comprises five risk categories (high risk, medium-high risk, medium risk, medium-low risk, low risk) based on the probability of return
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransferAuthorizationRiskLevel {
    #[serde(rename = "HIGH_RISK")]
    HighRisk,
    #[serde(rename = "MEDIUM_HIGH_RISK")]
    MediumHighRisk,
    #[serde(rename = "MEDIUM_RISK")]
    MediumRisk,
    #[serde(rename = "MEDIUM_LOW_RISK")]
    MediumLowRisk,
    #[serde(rename = "LOW_RISK")]
    LowRisk,
}
