use serde::{Serialize, Deserialize};
///The registration status for the end customer's application.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PartnerEndCustomerOAuthInstitutionApplicationStatus {
    #[serde(rename = "NOT_STARTED")]
    NotStarted,
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "ATTENTION_REQUIRED")]
    AttentionRequired,
}
