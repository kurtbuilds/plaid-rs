use serde::{Serialize, Deserialize};
///The outcome status for the associated Identity Verification attempt's `verify_sms` step. This field will always have the same value as `steps.verify_sms`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VerifySmsDetailsStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}
