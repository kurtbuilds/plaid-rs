use serde::{Serialize, Deserialize};
///A status enum indicating whether autofill succeeded or failed.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IdentityVerificationAutofillStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}
