use serde::{Serialize, Deserialize};
use super::BeaconUser;
///The response schema for `/beacon/user/history/list`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BeaconUserHistoryListResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beacon_users: Vec<BeaconUser>,
    ///An identifier that determines which page of results you receive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for BeaconUserHistoryListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
