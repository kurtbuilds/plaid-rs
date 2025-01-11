use serde::{Serialize, Deserialize};
///Contains data about the connected Item.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CraNetworkInsightsItem {
    ///The ID for the institution the user linked.
    pub institution_id: String,
    ///The name of the institution the user linked.
    pub institution_name: String,
    ///The identifier for the Item.
    pub item_id: String,
}
impl std::fmt::Display for CraNetworkInsightsItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
