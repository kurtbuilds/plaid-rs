use serde::{Serialize, Deserialize};
use super::{
    SelfieAnalysisDocumentComparison, SelfieAnalysisFacialAnalysis,
    SelfieAnalysisLivenessCheck,
};
///High level descriptions of how the associated selfie was processed. If a selfie fails verification, the details in the `analysis` object should help clarify why the selfie was rejected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfieAnalysis {
    ///Information about the comparison between the selfie and the document (if documentary verification also ran).
    pub document_comparison: SelfieAnalysisDocumentComparison,
    ///Analysis of the facial features of the selfie when compared to the face in the uploaded document, if one is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facial_analysis: Option<SelfieAnalysisFacialAnalysis>,
    ///Assessment of whether the selfie capture is of a real human being, as opposed to a picture of a human on a screen, a picture of a paper cut out, someone wearing a mask, or a deepfake.
    pub liveness_check: SelfieAnalysisLivenessCheck,
}
impl std::fmt::Display for SelfieAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
