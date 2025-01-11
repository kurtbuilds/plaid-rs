use serde::{Serialize, Deserialize};
use super::{ProfileIdentity, ProfileItem};
///ProfileGetResponse defines the response schema for `/profile/get`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileGetResponse {
    ///ProfileIdentity defines the identity data permissioned by the end user during the authorization flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ProfileIdentity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ProfileItem>,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProfileGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
