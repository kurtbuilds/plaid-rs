use serde::{Serialize, Deserialize};
///Specifies options for initializing Link for use with the Transfer product.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkTokenCreateRequestTransfer {
    ///The `id` returned by the `/transfer/authorization/create` endpoint. Used to indicate Link session to complete required user action in order to make a decision for the authorization. If set, `access_token` can be omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization_id: Option<String>,
    ///The `id` returned by the `/transfer/intent/create` endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    ///The `payment_profile_token` returned by the `/payment_profile/create` endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_profile_token: Option<String>,
}
impl std::fmt::Display for LinkTokenCreateRequestTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
