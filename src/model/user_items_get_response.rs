use serde::{Serialize, Deserialize};
use super::Item;
///UserItemsGetResponse defines the response schema for `/user/items/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserItemsGetResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Item>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for UserItemsGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
