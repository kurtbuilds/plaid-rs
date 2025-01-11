use serde::{Serialize, Deserialize};
///UserThirdPartyTokenCreateResponse defines the response schema for `/user/third_party_token/create`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserThirdPartyTokenCreateResponse {
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    ///The third-party user token associated with the requested User data.
    pub third_party_user_token: String,
}
impl std::fmt::Display for UserThirdPartyTokenCreateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
