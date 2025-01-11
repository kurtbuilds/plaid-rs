use serde::{Serialize, Deserialize};
///An outcome status for this specific selfie. Distinct from the overall `selfie_check.status` that summarizes the verification outcome from one or more selfies.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SelfieStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}
