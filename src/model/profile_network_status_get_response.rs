use serde::{Serialize, Deserialize};
use super::ProfileNetworkStatusGetNetworkStatus;
///ProfileNetworkStatusGetResponse defines the response schema for `/profile/network_status/get`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileNetworkStatusGetResponse {
    ///Enum representing the overall network status of the user
    pub network_status: ProfileNetworkStatusGetNetworkStatus,
    ///A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}
impl std::fmt::Display for ProfileNetworkStatusGetResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
