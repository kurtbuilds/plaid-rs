use serde::{Serialize, Deserialize};
use super::{CraPartnerInsightsItem, CraPartnerInsightsPrism};
///The Partner Insights report of the bank data for an end user.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraPartnerInsights {
    ///The time when the Partner Insights report was generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_time: Option<chrono::DateTime<chrono::Utc>>,
    ///The list of Items used in the report along with the associated metadata about the Item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CraPartnerInsightsItem>>,
    ///The Prism Data insights for the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prism: Option<CraPartnerInsightsPrism>,
    ///A unique identifier associated with the Partner Insights object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}
impl std::fmt::Display for CraPartnerInsights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
