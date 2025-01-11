use serde::{Serialize, Deserialize};
///Information about the comparison between the selfie and the document (if documentary verification also ran).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SelfieAnalysisDocumentComparison {
    #[serde(rename = "match")]
    Match,
    #[serde(rename = "no_match")]
    NoMatch,
    #[serde(rename = "no_input")]
    NoInput,
}
