use serde::{Serialize, Deserialize};
///An Item created during a Layer authorization session.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserAccountItem {
    ///The access token associated with the Item data is being requested for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    ///The Plaid Item ID. The `item_id` is always unique; linking the same account at the same institution twice will result in two Items with different `item_id` values. Like all Plaid identifiers, the `item_id` is case-sensitive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}
impl std::fmt::Display for UserAccountItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
