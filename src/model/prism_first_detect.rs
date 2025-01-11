use serde::{Serialize, Deserialize};
use super::PrismCashScoreMetadata;
///The data from the FirstDetect product returned by Prism Data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrismFirstDetect {
    ///The error returned by Prism for this product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    ///An object containing metadata about the provided transactions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PrismCashScoreMetadata>,
    ///The version of Prism Data's FirstDetect model used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    ///The reasons for an individual having risk according to the FirstDetect score.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<String>>,
    ///The score returned by Prism Data. Ranges from 1-999, with higher score indicating lower risk.
    pub score: i64,
    ///The version of Prism Data's FirstDetect model used. This field is deprecated in favor of `model_version`.
    pub version: i64,
}
impl std::fmt::Display for PrismFirstDetect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
