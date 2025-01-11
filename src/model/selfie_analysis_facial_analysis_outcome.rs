use serde::{Serialize, Deserialize};
///Outcome of the facial analysis for a specific facial feature.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SelfieAnalysisFacialAnalysisOutcome {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}
