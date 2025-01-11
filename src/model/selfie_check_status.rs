use serde::{Serialize, Deserialize};
///The outcome status for the associated Identity Verification attempt's `selfie_check` step. This field will always have the same value as `steps.selfie_check`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SelfieCheckStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}
