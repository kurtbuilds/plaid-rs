use serde::{Serialize, Deserialize};
///The status of a step in the Identity Verification process.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IdentityVerificationStepStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "waiting_for_prerequisite")]
    WaitingForPrerequisite,
    #[serde(rename = "not_applicable")]
    NotApplicable,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "pending_review")]
    PendingReview,
    #[serde(rename = "manually_approved")]
    ManuallyApproved,
    #[serde(rename = "manually_rejected")]
    ManuallyRejected,
}
