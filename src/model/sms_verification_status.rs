use serde::{Serialize, Deserialize};
///The outcome status for the individual SMS verification.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SmsVerificationStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "canceled")]
    Canceled,
}
