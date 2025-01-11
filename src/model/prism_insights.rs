use serde::{Serialize, Deserialize};
use super::PrismInsightsResult;
///The data from the Insights product returned by Prism Data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrismInsights {
    ///The error returned by Prism for this product.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    ///The Insights Result object is a map of cash flow attributes, where the key is a string, and the value is a float or string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<PrismInsightsResult>,
    ///The version of Prism Data's insights model used.
    pub version: i64,
}
impl std::fmt::Display for PrismInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
