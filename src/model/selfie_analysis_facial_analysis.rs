use serde::{Serialize, Deserialize};
use super::SelfieAnalysisFacialAnalysisOutcome;
///Analysis of the facial features of the selfie when compared to the face in the uploaded document, if one is present.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfieAnalysisFacialAnalysis {
    ///Outcome of the facial analysis for a specific facial feature.
    pub forehead: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub jaw: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub left_brow: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub left_cheek: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub left_eye: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub middle_forehead: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub mouth: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub nose: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub philtrum: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub right_brow: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub right_cheek: SelfieAnalysisFacialAnalysisOutcome,
    ///Outcome of the facial analysis for a specific facial feature.
    pub right_eye: SelfieAnalysisFacialAnalysisOutcome,
}
impl std::fmt::Display for SelfieAnalysisFacialAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
