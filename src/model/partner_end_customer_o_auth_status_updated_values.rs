use serde::{Serialize, Deserialize};
///The OAuth status of the update
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PartnerEndCustomerOAuthStatusUpdatedValues {
    #[serde(rename = "not-started")]
    NotStarted,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "attention-required")]
    AttentionRequired,
}
