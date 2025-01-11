use serde::{Serialize, Deserialize};
use super::IncomeVerificationDocParsingConfig;
///Identity object used to specify document upload
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateIdentity {
    ///An array of `account_ids`. Currently can only contain one `account_id`. Must be populated if using Document Upload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    ///Used to specify whether the Link session is Identity Document Upload
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_document_upload: Option<bool>,
    ///An array of parsing configurations. Valid parsing configurations are `ocr` and `risk_signals`. If parsing configurations are omitted, defaults to `ocr`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parsing_configs: Option<Vec<IncomeVerificationDocParsingConfig>>,
}
impl std::fmt::Display for LinkTokenCreateIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
