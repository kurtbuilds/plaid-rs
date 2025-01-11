use serde::{Serialize, Deserialize};
///Assessment of whether the selfie capture is of a real human being, as opposed to a picture of a human on a screen, a picture of a paper cut out, someone wearing a mask, or a deepfake.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SelfieAnalysisLivenessCheck {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}
