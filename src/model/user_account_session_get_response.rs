use serde::{Serialize, Deserialize};
use super::{UserAccountIdentity, UserAccountItem};
///UserAccountSessionGetResponse defines the response schema for `/user_account/session/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserAccountSessionGetResponse {
    ///The identity data permissioned by the end user during the authorization flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<UserAccountIdentity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<UserAccountItem>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for UserAccountSessionGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
