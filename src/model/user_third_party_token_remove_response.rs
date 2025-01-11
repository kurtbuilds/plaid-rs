use serde::{Serialize, Deserialize};
///UserThirdPartyTokenCreateResponse defines the response schema for `/user/third_party_token/remove`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserThirdPartyTokenRemoveResponse {
    ///`true` if the third-party user token was successfully removed.
    pub removed: bool,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for UserThirdPartyTokenRemoveResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
