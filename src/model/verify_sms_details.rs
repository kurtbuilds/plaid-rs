use serde::{Serialize, Deserialize};
use super::{SmsVerification, VerifySmsDetailsStatus};
///Additional information for the `verify_sms` step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifySmsDetails {
    ///The outcome status for the associated Identity Verification attempt's `verify_sms` step. This field will always have the same value as `steps.verify_sms`.
    pub status: VerifySmsDetailsStatus,
    ///An array where each entry represents a verification attempt for the `verify_sms` step. Each entry represents one user-submitted phone number. Phone number edits, and in some cases error handling due to edge cases like rate limiting, may generate additional verifications.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verifications: Vec<SmsVerification>,
}
impl std::fmt::Display for VerifySmsDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
