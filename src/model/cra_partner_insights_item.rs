use serde::{Serialize, Deserialize};
use super::CraPartnerInsightsItemAccount;
///The details and metadata for an end user's Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraPartnerInsightsItem {
    ///A list of accounts in the item
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<CraPartnerInsightsItemAccount>>,
    ///The ID for the institution that the user linked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    ///The name of the institution the user linked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    ///The identifier for the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}
impl std::fmt::Display for CraPartnerInsightsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
