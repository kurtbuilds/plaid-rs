use serde::{Serialize, Deserialize};
///An outcome status for this specific document submission. Distinct from the overall `documentary_verification.status` that summarizes the verification outcome from one or more documents.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DocumentStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "manually_approved")]
    ManuallyApproved,
}
