use serde::{Serialize, Deserialize};
///The Insights Result object is a map of cash flow attributes, where the key is a string, and the value is a float or string.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrismInsightsResult {}
impl std::fmt::Display for PrismInsightsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
