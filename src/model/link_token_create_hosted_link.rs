use serde::{Serialize, Deserialize};
use super::HostedLinkDeliveryMethod;
///Configuration parameters for Hosted Link. To enable the session for Hosted Link, send this object in the request. It can be empty.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkTokenCreateHostedLink {
    ///URI that Hosted Link will redirect to upon completion of the Link flow. This will only occur in Hosted Link sessions, not in other implementation methods.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_redirect_uri: Option<String>,
    /**How Plaid should deliver the Plaid Link session to the customer. Only available to customers enabled for Link Delivery (beta). To request Link Delivery access, contact your account manager.
'sms' will deliver via SMS. Must pass `user.phone_number`.
'email' will deliver via email. Must pass `user.email_address`. In the Sandbox environment, this field will be ignored; use the Production environment to test Link Delivery instead.*/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<HostedLinkDeliveryMethod>,
    ///This indicates whether the client is opening hosted Link in a mobile app in an out of process web view (OOPWV) (i.e., an `AsWebAuthenticationSession` / `SFSafariViewController` or Android Custom Tab).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_mobile_app: Option<bool>,
    ///How many seconds the link will be valid for. Must be positive. Cannot be longer than 21 days. The default lifetime is 7 days for links delivered by email, 1 day for links delivered via SMS, and 30 minutes for links not sent via Plaid Link delivery. This parameter will override the value of all three link types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url_lifetime_seconds: Option<i64>,
}
impl std::fmt::Display for LinkTokenCreateHostedLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
