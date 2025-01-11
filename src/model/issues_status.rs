use serde::{Serialize, Deserialize};
///The current status of the issue.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IssuesStatus {
    #[serde(rename = "REPORTED")]
    Reported,
    #[serde(rename = "AWAITING_RESOLUTION")]
    AwaitingResolution,
    #[serde(rename = "FIX_IN_PROGRESS")]
    FixInProgress,
    #[serde(rename = "FIX_PENDING_VALIDATION")]
    FixPendingValidation,
    #[serde(rename = "CANNOT_FIX")]
    CannotFix,
    #[serde(rename = "RESOLVED")]
    Resolved,
}
