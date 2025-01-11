use serde::{Serialize, Deserialize};
use super::{PrismCashScoreVersion, PrismFirstDetectVersion, PrismInsightsVersion};
///The versions of Prism products to evaluate
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PrismVersions {
    ///The version of Prism CashScore. If not specified, will default to v3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cashscore: Option<PrismCashScoreVersion>,
    ///The version of Prism FirstDetect. If not specified, will default to v3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub firstdetect: Option<PrismFirstDetectVersion>,
    ///The version of Prism Insights. If not specified, will default to v3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insights: Option<PrismInsightsVersion>,
}
impl std::fmt::Display for PrismVersions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
