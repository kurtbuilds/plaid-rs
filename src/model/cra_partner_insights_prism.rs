use serde::{Serialize, Deserialize};
use super::{PrismCashScore, PrismFirstDetect, PrismInsights};
///The Prism Data insights for the user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraPartnerInsightsPrism {
    ///The data from the CashScore® product returned by Prism Data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_score: Option<PrismCashScore>,
    ///The data from the FirstDetect product returned by Prism Data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_detect: Option<PrismFirstDetect>,
    ///The data from the Insights product returned by Prism Data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insights: Option<PrismInsights>,
    ///Details on whether the Prism Data attributes succeeded or failed to be generated.
    pub status: String,
}
impl std::fmt::Display for CraPartnerInsightsPrism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
