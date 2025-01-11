use serde::{Serialize, Deserialize};
///Indicates an Item's micro-deposit-based verification or database verification status.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LinkDeliveryVerificationStatus {
    #[serde(rename = "automatically_verified")]
    AutomaticallyVerified,
    #[serde(rename = "pending_automatic_verification")]
    PendingAutomaticVerification,
    #[serde(rename = "pending_manual_verification")]
    PendingManualVerification,
    #[serde(rename = "manually_verified")]
    ManuallyVerified,
    #[serde(rename = "verification_expired")]
    VerificationExpired,
    #[serde(rename = "verification_failed")]
    VerificationFailed,
    #[serde(rename = "database_matched")]
    DatabaseMatched,
    #[serde(rename = "database_insights_pending")]
    DatabaseInsightsPending,
}
